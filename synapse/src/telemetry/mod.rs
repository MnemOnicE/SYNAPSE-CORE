use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("Failed to open or create telemetry file: {0}")]
    FileError(#[from] io::Error),
    #[error("Failed to map telemetry file")]
    MapError,
}

/// A shared memory telemetry interface to read the Python agent's
/// cycle completion rate to calculate dQ/dt for the event horizon.
pub struct AgentTelemetry {
    mmap: MmapMut,
}

impl AgentTelemetry {
    /// Creates or opens a shared memory control file.
    /// Expects a file size of at least 4 bytes (to hold a u32 for the rate).
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, TelemetryError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;

        // Ensure the file is at least 4 bytes large
        if file.metadata()?.len() < 4 {
            file.set_len(4)?;
        }

        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

        Ok(Self { mmap })
    }

    /// Reads the real-time processing rate (e.g., cycles per second or moving average)
    /// published by the Python agent in the Termux environment.
    /// The value is read atomically to avoid tearing.
    pub fn get_processing_rate(&self) -> u32 {
        let ptr_addr = self.mmap.as_ptr() as usize;
        assert!(
            ptr_addr % std::mem::align_of::<AtomicU32>() == 0,
            "mmap address is not aligned to AtomicU32"
        );
        let ptr = self.mmap.as_ptr() as *const AtomicU32;
        let atomic_ref = unsafe { &*ptr };
        atomic_ref.load(Ordering::Acquire)
    }

    /// Helper for testing to simulate Python agent writes
    pub fn set_processing_rate(&mut self, rate: u32) {
        let ptr_addr = self.mmap.as_mut_ptr() as usize;
        assert!(
            ptr_addr % std::mem::align_of::<AtomicU32>() == 0,
            "mmap address is not aligned to AtomicU32"
        );
        let ptr = self.mmap.as_mut_ptr() as *mut AtomicU32;
        let atomic_ref = unsafe { &*ptr };
        atomic_ref.store(rate, Ordering::Release);
    }
}
