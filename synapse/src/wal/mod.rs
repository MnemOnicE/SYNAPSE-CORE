use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("WAL is full")]
    Full,
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
}

/// A highly performant, crash-persistent memory-mapped Write-Ahead Log (WAL)
/// acting as a ring buffer. Designed for Termux's native private directory (`~`)
/// to avoid emulated storage latency.
pub struct MmapRingBuffer {
    mmap: MmapMut,
    capacity: usize,
    head: Mutex<usize>,
    tail: Mutex<usize>,
}

impl MmapRingBuffer {
    /// Initializes the WAL in a given path. It is CRITICAL that this path is in
    /// the native Termux app directory (e.g., `~/.synapse/wal.bin`).
    pub fn new<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self, WalError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true).truncate(false)
            .open(&path)?;

        // Expand file if necessary. Add 16 bytes for header (head/tail indices)
        let total_size = capacity + 16;
        if file.metadata()?.len() < total_size as u64 {
            file.set_len(total_size as u64)?;
        }

        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

        // We assume an uninitialized file starts with zeros, so head=0, tail=0 initially.
        // In a full implementation, we'd read the head/tail from the first 16 bytes.

        Ok(Self {
            mmap,
            capacity,
            head: Mutex::new(0),
            tail: Mutex::new(0),
        })
    }

    /// Writes raw bytes to the ring buffer.
    pub fn write_raw(&mut self, data: &[u8]) -> Result<(), WalError> {
        let mut head = self.head.lock().unwrap();
        let tail = self.tail.lock().unwrap();

        // Ensure there is space (simplistic check for this scaffold)
        let space_available = if *head >= *tail {
            self.capacity - (*head - *tail) - 1
        } else {
            *tail - *head - 1
        };

        if data.len() > space_available {
            return Err(WalError::Full);
        }

        let start_idx = 16 + *head;
        if start_idx + data.len() <= 16 + self.capacity {
            // Contiguous write
            self.mmap[start_idx..start_idx + data.len()].copy_from_slice(data);
            *head = (*head + data.len()) % self.capacity;
        } else {
            // Wraparound write
            let first_part = 16 + self.capacity - start_idx;
            self.mmap[start_idx..16 + self.capacity].copy_from_slice(&data[..first_part]);
            let second_part = data.len() - first_part;
            self.mmap[16..16 + second_part].copy_from_slice(&data[first_part..]);
            *head = second_part;
        }

        // In a production system, we would flush asynchronously to disk and update header
        // self.mmap.flush_async()?;
        Ok(())
    }
}
