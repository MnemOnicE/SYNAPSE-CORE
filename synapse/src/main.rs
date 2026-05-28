use bytes::BytesMut;
use serde::Deserialize;
use std::io::Read;
use std::os::unix::net::UnixListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use synapse::horizon::bus::HorizonBus;
use synapse::telemetry::AgentTelemetry;
use synapse::wal::MmapRingBuffer;
use tempfile::NamedTempFile;

#[allow(dead_code)]
#[derive(Deserialize)]
struct PeekingHeader<'a> {
    event_category: &'a str,
    #[serde(borrow)]
    data: &'a serde_json::value::RawValue,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Project SYNAPSE Horizon Engine...");

    // 1. Setup Non-Blocking UDP Telemetry
    let telemetry = Arc::new(Mutex::new(AgentTelemetry::new()?));

    // 2. Setup the Mmap Ring Buffer (WAL)
    let wal_file = NamedTempFile::new()?;
    let wal = Arc::new(Mutex::new(MmapRingBuffer::new(
        wal_file.path(),
        1024 * 1024,
    )?));

    // 3. Initialize the Horizon Bus
    let bus = Arc::new(Mutex::new(HorizonBus::new(telemetry.clone(), wal.clone())));

    // Simulate Python agent heartbeats via UDP
    thread::spawn(|| {
        let mut bind_attempts = 0;
        let socket = loop {
            match std::net::UdpSocket::bind("127.0.0.1:0") {
                Ok(s) => break s,
                Err(e) => {
                    bind_attempts += 1;
                    eprintln!(
                        "Warning: Failed to bind simulation UDP socket (attempt {}): {}",
                        bind_attempts, e
                    );
                    if bind_attempts >= 5 {
                        eprintln!(
                            "Error: Simulation heartbeat thread terminating after 5 failed bind attempts."
                        );
                        return;
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
        };

        loop {
            // Heartbeat indicating 150 cycles/sec
            if let Err(e) = socket.send_to(b"150.0", "127.0.0.1:9000") {
                eprintln!("Warning: Failed to send simulated heartbeat: {}", e);
            }
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Simulated background bus evaluation loop
    let bus_eval = bus.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(50));
            let mut bus = bus_eval.lock().unwrap();
            bus.evaluate_horizon();
        }
    });

    // 4. Bind UDS Listener (Zero-Hop Ingress)
    // In a real Termux deployment, this would be `~/.synapse/ingress.sock`
    let socket_path = "/data/data/com.termux/files/home/.synapse/ingress.sock";
    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path)?;
    // Enforce The Permissions Law: 0600
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o600))?;
    }
    println!("Listening on UDS: {}", socket_path);

    // Main UDS Acceptance Loop
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let bus_clone = bus.clone();
                let wal_clone = wal.clone();

                thread::spawn(move || {
                    let mut buf = vec![0; 4096];
                    if let Ok(size) = stream.read(&mut buf) {
                        let mut bytes_mut = BytesMut::new();
                        bytes_mut.extend_from_slice(&buf[..size]);
                        let bytes = bytes_mut.freeze();

                        // Parse header with zero-copy
                        if let Ok(header) = serde_json::from_slice::<PeekingHeader>(&bytes) {
                            println!("Routed Event Category: {}", header.event_category);

                            // Send full raw payload to WAL
                            if let Ok(wal) = wal_clone.lock() {
                                let _ = wal.write_raw(&bytes);
                            }

                            // Increment queue depth
                            if let Ok(mut bus) = bus_clone.lock() {
                                bus.increment_queue();
                            }
                        }
                    }
                });
            }
            Err(err) => {
                eprintln!("UDS Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}
