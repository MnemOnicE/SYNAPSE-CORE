use std::net::UdpSocket;
use std::time::Instant;

/// A non-blocking UDP telemetry interface to read the Python agent's
/// cycle completion rate to calculate dQ/dt for the event horizon.
pub struct AgentTelemetry {
    socket: UdpSocket,
    current_rate: f32,
    last_heartbeat: Instant,
    last_update: Instant,
}

impl AgentTelemetry {
    /// Binds a non-blocking UdpSocket listening on 127.0.0.1:9000
    pub fn new() -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("127.0.0.1:9000")?;
        socket.set_nonblocking(true)?;

        let now = Instant::now();
        Ok(Self {
            socket,
            current_rate: 0.1, // Floor value
            last_heartbeat: now,
            last_update: now,
        })
    }

    /// Reads all pending UDP packets, updating the processing rate via EMA if a new
    /// heartbeat is found. Applies exponential decay if the agent is silent for > 250ms.
    pub fn poll_and_decay(&mut self) -> f32 {
        let mut buf = [0u8; 64]; // Small buffer for simple scalar strings
        let mut latest_heartrate: Option<f32> = None;

        // Drain the socket buffer
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    if let Ok(msg) = std::str::from_utf8(&buf[..amt]) {
                        if let Ok(val) = msg.trim().parse::<f32>() {
                            latest_heartrate = Some(val);
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(_) => {
                    // Ignore other socket errors for resilience
                    break;
                }
            }
        }

        let now = Instant::now();

        if let Some(rate) = latest_heartrate {
            // New data arrived, apply EMA for recovery to prevent whiplash
            self.current_rate = (rate * 0.2) + (self.current_rate * 0.8);
            if self.current_rate < 0.1 {
                self.current_rate = 0.1;
            }
            self.last_heartbeat = now;
            self.last_update = now;
        } else {
            // No new data. Check for decay
            let elapsed_since_heartbeat = now.duration_since(self.last_heartbeat).as_millis();
            if elapsed_since_heartbeat > 250 {
                let elapsed_since_update = now.duration_since(self.last_update).as_secs_f32();
                let decay_factor = f32::powf(0.9, elapsed_since_update / 0.05);
                self.current_rate *= decay_factor;
                if self.current_rate < 0.1 {
                    self.current_rate = 0.1;
                }
            }
            self.last_update = now;
        }

        self.current_rate
    }
}
