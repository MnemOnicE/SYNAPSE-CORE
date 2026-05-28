use crate::telemetry::AgentTelemetry;
use crate::wal::MmapRingBuffer;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// The State of the Horizon Message Bus
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BusState {
    NormalFlow, // The Bulk (3D space)
    Folded,     // Dimensionally Folded (Holographic surface)
}

pub struct HorizonBus {
    state: BusState,
    pub telemetry: Arc<Mutex<AgentTelemetry>>,
    pub wal: Arc<Mutex<MmapRingBuffer>>,
    queue_depth: usize,
    last_check: Instant,
    last_queue_depth: usize,
}

impl HorizonBus {
    pub fn new(telemetry: Arc<Mutex<AgentTelemetry>>, wal: Arc<Mutex<MmapRingBuffer>>) -> Self {
        Self {
            state: BusState::NormalFlow,
            telemetry,
            wal,
            queue_depth: 0,
            last_check: Instant::now(),
            last_queue_depth: 0,
        }
    }

    /// Evaluates the threshold logic: dQ/dt > Processing Rate
    /// If the saturation point is breached, it transitions the bus to Folded mode.
    pub fn evaluate_horizon(&mut self) -> BusState {
        let now = Instant::now();
        let dt = now.duration_since(self.last_check).as_secs_f32();

        if dt > 0.05 {
            // Evaluate every 50ms
            let dq = (self.queue_depth as isize - self.last_queue_depth as isize) as f32;
            let dq_dt = dq / dt;

            // Poll UDP and apply EMA/decay to get the latest processing rate
            let processing_rate = {
                let mut telemetry = self.telemetry.lock().unwrap();
                telemetry.poll_and_decay()
            };

            // The Schwarzschild limit logic
            if dq_dt > processing_rate {
                if self.state == BusState::NormalFlow {
                    println!(
                        "[HORIZON] Threshold reached (dQ/dt: {}, Rate: {}). Folding dimensions.",
                        dq_dt, processing_rate
                    );
                    self.state = BusState::Folded;
                }
            } else if dq_dt < (processing_rate * 0.5) && self.state == BusState::Folded {
                // Drop below a safe threshold to unfold
                println!("[HORIZON] Load stabilized. Unfolding.");
                self.state = BusState::NormalFlow;
            }

            self.last_check = now;
            self.last_queue_depth = self.queue_depth;
        }

        self.state
    }

    pub fn increment_queue(&mut self) {
        self.queue_depth += 1;
    }

    pub fn decrement_queue(&mut self) {
        if self.queue_depth > 0 {
            self.queue_depth -= 1;
        }
    }

    pub fn current_state(&self) -> BusState {
        self.state
    }
}

impl HorizonBus {
    /// Asymmetric Recovery logic: The TTL (Time-To-Live) cull.
    /// Evaluates whether a folded payload's header is still fresh enough to warrant
    /// unfolding and processing the raw payload. Stale data (Hawking radiation) is dropped.
    pub fn should_unfold<T>(
        &self,
        header: &crate::horizon::HolographicHeader<T>,
        max_age_ms: i64,
    ) -> bool {
        let now = chrono::Utc::now();
        let age = now
            .signed_duration_since(header.timestamp)
            .num_milliseconds();

        if age < 0 {
            println!(
                "[HORIZON] Culling future-dated frame (age: {}ms). Invalid TTL.",
                age
            );
            false
        } else if age > max_age_ms {
            println!(
                "[HORIZON] Culling stale frame (age: {}ms). Dropping raw volume.",
                age
            );
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::horizon::HolographicHeader;
    use chrono::{Duration, Utc};
    use std::sync::OnceLock;

    static BUS: OnceLock<HorizonBus> = OnceLock::new();

    fn get_bus() -> &'static HorizonBus {
        BUS.get_or_init(|| {
            // Use dummy implementations or minimal setups for testing should_unfold
            // Since AgentTelemetry binds to a port, we'll try to let it bind, but since
            // should_unfold does not touch telemetry or wal, we just need ANY valid instance.
            // If AgentTelemetry::new() fails (e.g., port in use), we'll panic, but it's
            // safe for isolated test runs.
            let telemetry = Arc::new(Mutex::new(
                AgentTelemetry::new().expect("Failed to initialize AgentTelemetry for test bus")
            ));

            let temp_dir = tempfile::tempdir().expect("Failed to create temp dir for test bus WAL");
            let wal_path = temp_dir.path().join("test_wal.bin");
            let wal = Arc::new(Mutex::new(
                MmapRingBuffer::new(&wal_path, 1024).expect("Failed to initialize WAL for test bus")
            ));

            HorizonBus::new(telemetry, wal)
        })
    }

    #[test]
    fn test_should_unfold_fresh_frame() {
        let bus = get_bus();
        let mut header = HolographicHeader::new((), true, 0.5);
        // Set timestamp to 50ms ago
        header.timestamp = Utc::now() - Duration::try_milliseconds(50).unwrap();

        // Max age is 100ms, 50ms is fresh
        assert!(bus.should_unfold(&header, 100));
    }

    #[test]
    fn test_should_unfold_stale_frame() {
        let bus = get_bus();
        let mut header = HolographicHeader::new((), true, 0.5);
        // Set timestamp to 150ms ago
        header.timestamp = Utc::now() - Duration::try_milliseconds(150).unwrap();

        // Max age is 100ms, 150ms is stale
        assert!(!bus.should_unfold(&header, 100));
    }

    #[test]
    fn test_should_unfold_future_frame() {
        let bus = get_bus();
        let mut header = HolographicHeader::new((), true, 0.5);
        // Set timestamp to 50ms in the future
        header.timestamp = Utc::now() + Duration::try_milliseconds(50).unwrap();

        assert!(!bus.should_unfold(&header, 100));
    }

    #[test]
    fn test_should_unfold_edge_cases() {
        let bus = get_bus();

        // Test exact 0 age (or very close to it)
        let mut header = HolographicHeader::new((), true, 0.5);
        header.timestamp = Utc::now();
        assert!(bus.should_unfold(&header, 100));


        // Max age threshold (testing exactly at boundary can be flaky due to execution time)
        // so we test right before the boundary (e.g., 90ms for a 100ms max age)
        let mut near_boundary_header = HolographicHeader::new((), true, 0.5);
        near_boundary_header.timestamp = Utc::now() - Duration::try_milliseconds(90).unwrap();
        assert!(bus.should_unfold(&near_boundary_header, 100));
    }
}
