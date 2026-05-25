use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use synapse::horizon::{
    Foldable, HolographicHeader,
    bus::{BusState, HorizonBus},
};
use synapse::payloads::{LidarPoint, LidarPointCloud};
use synapse::telemetry::AgentTelemetry;
use synapse::wal::MmapRingBuffer;
use tempfile::NamedTempFile;

fn main() {
    println!("Initializing Project SYNAPSE Horizon Engine...");

    // 1. Setup Backpressure Telemetry
    // In a real Termux deployment, this would be a known path in the private app dir.
    let telemetry_file = NamedTempFile::new().expect("Failed to create temp telemetry file");
    let mut telemetry_writer = AgentTelemetry::new(telemetry_file.path()).unwrap();
    let telemetry_reader = Arc::new(AgentTelemetry::new(telemetry_file.path()).unwrap());

    // 2. Setup the Mmap Ring Buffer (WAL)
    let wal_file = NamedTempFile::new().expect("Failed to create temp WAL file");
    let wal = Arc::new(Mutex::new(
        MmapRingBuffer::new(wal_file.path(), 1024 * 1024).unwrap(),
    ));

    // 3. Initialize the Horizon Bus
    let mut bus = HorizonBus::new(telemetry_reader.clone(), wal.clone());

    // 4. Simulate the system load and Horizon crossing
    println!("Simulating low load (Bulk spacetime)...");

    // Simulate Python agent processing 100 packets/sec
    telemetry_writer.set_processing_rate(100);

    for _ in 0..5 {
        bus.increment_queue();
    }

    thread::sleep(Duration::from_millis(60));
    bus.evaluate_horizon();

    println!("Simulating load spike (Event Horizon)...");

    for _ in 0..20 {
        bus.increment_queue(); // dQ/dt will be high
    }

    thread::sleep(Duration::from_millis(60));
    bus.evaluate_horizon();

    if bus.current_state() == BusState::Folded {
        println!("Bus is folded. Creating Holographic Header for a massive LiDAR point cloud.");

        let cloud = LidarPointCloud {
            points: vec![
                LidarPoint {
                    x: 0.1,
                    y: 0.2,
                    z: 0.3,
                },
                LidarPoint {
                    x: 10.1,
                    y: 5.2,
                    z: -1.3,
                },
                LidarPoint {
                    x: -2.1,
                    y: 3.2,
                    z: 7.3,
                },
            ],
        };

        let (folded_payload, entropy) = cloud.fold();
        let header = HolographicHeader::new(folded_payload, true, entropy);

        println!("Generated Header: {:?}", header);

        // Simulating the TTL check
        let is_fresh = bus.should_unfold(&header, 500); // Max age 500ms
        println!("Is frame fresh enough to unfold? {}", is_fresh);
    }

    println!("Simulating recovery...");
    bus.decrement_queue();
    bus.decrement_queue();
    bus.decrement_queue();

    thread::sleep(Duration::from_millis(60));
    bus.evaluate_horizon();

    println!("SYNAPSE Execution Complete.");
}
