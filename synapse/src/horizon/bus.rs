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
    telemetry: Arc<AgentTelemetry>,
    pub wal: Arc<Mutex<MmapRingBuffer>>,
    queue_depth: usize,
    last_check: Instant,
    last_queue_depth: usize,
}

impl HorizonBus {
    pub fn new(telemetry: Arc<AgentTelemetry>, wal: Arc<Mutex<MmapRingBuffer>>) -> Self {
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

            // Read Python agent's processing rate from shared memory telemetry
            let processing_rate = self.telemetry.get_processing_rate() as f32;

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

        if age > max_age_ms {
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
