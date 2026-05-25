use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// The fundamental trait for polymorphically folding data across the Horizon.
/// Requires Send + Sync to ensure thread safety during high-frequency ingestion.
pub trait Foldable: Send + Sync + Debug {
    type FoldedPayload: Send + Sync + Debug + Serialize + for<'a> Deserialize<'a>;

    /// Compresses ("folds") the full data volume into a unified scalar or
    /// summary representation along with calculated entropy loss.
    fn fold(&self) -> (Self::FoldedPayload, f32);

    /// Recovers ("unfolds") the raw data back from the summary representation,
    /// given the folded payload. If lossy, it's acknowledged by the system.
    fn unfold(header: &HolographicHeader<Self::FoldedPayload>) -> Self;
}

/// The structure containing the dimensional "boundary state" (the interface layer)
/// instead of the redundant 3D volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolographicHeader<T> {
    pub is_folded: bool,
    pub entropy_loss: f32,
    pub timestamp: DateTime<Utc>,
    pub payload: T,
}

impl<T> HolographicHeader<T> {
    pub fn new(payload: T, is_folded: bool, entropy_loss: f32) -> Self {
        Self {
            is_folded,
            entropy_loss,
            timestamp: Utc::now(),
            payload,
        }
    }
}
pub mod bus;
