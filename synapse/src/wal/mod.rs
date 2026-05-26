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

pub struct Indices {
    pub head: usize,
    pub tail: usize,
}

/// A highly performant, crash-persistent memory-mapped Write-Ahead Log (WAL)
/// acting as a ring buffer. Designed for Termux's native private directory (`~`)
/// to avoid emulated storage latency.
pub struct MmapRingBuffer {
    mmap: Mutex<MmapMut>,
    capacity: usize,
    indices: Mutex<Indices>,
}

impl MmapRingBuffer {
    /// Initializes the WAL in a given path. It is CRITICAL that this path is in
    /// the native Termux app directory (e.g., `~/.synapse/wal.bin`).
    pub fn new<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self, WalError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;

        // Expand file if necessary. Add 16 bytes for header (head/tail indices)
        let total_size = capacity + 16;
        if file.metadata()?.len() < total_size as u64 {
            file.set_len(total_size as u64)?;
        }

        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

        let indices = Self::load_indices(&mmap);

        Ok(Self {
            mmap: Mutex::new(mmap),
            capacity,
            indices: Mutex::new(indices),
        })
    }

    fn load_indices(mmap: &MmapMut) -> Indices {
        let mut head_bytes = [0u8; 8];
        let mut tail_bytes = [0u8; 8];
        head_bytes.copy_from_slice(&mmap[0..8]);
        tail_bytes.copy_from_slice(&mmap[8..16]);
        Indices {
            head: u64::from_le_bytes(head_bytes) as usize,
            tail: u64::from_le_bytes(tail_bytes) as usize,
        }
    }

    fn persist_indices(mmap: &mut MmapMut, indices: &Indices) {
        mmap[0..8].copy_from_slice(&(indices.head as u64).to_le_bytes());
        mmap[8..16].copy_from_slice(&(indices.tail as u64).to_le_bytes());
    }

    /// Writes raw bytes to the ring buffer.
    pub fn write_raw(&self, data: &[u8]) -> Result<(), WalError> {
        let mut indices = self.indices.lock().unwrap();
        let head = indices.head;
        let tail = indices.tail;

        // Ensure there is space: reserve 1 byte to distinguish full from empty
        let space_available = if head >= tail {
            (self.capacity - 1) - (head - tail)
        } else {
            tail - head - 1
        };

        if data.len() > space_available {
            return Err(WalError::Full);
        }

        let mut mmap = self.mmap.lock().unwrap();
        let start_idx = 16 + head;
        if start_idx + data.len() <= 16 + self.capacity {
            // Contiguous write
            mmap[start_idx..start_idx + data.len()].copy_from_slice(data);
            indices.head = (head + data.len()) % self.capacity;
        } else {
            // Wraparound write
            let first_part = 16 + self.capacity - start_idx;
            mmap[start_idx..16 + self.capacity].copy_from_slice(&data[..first_part]);
            let second_part = data.len() - first_part;
            mmap[16..16 + second_part].copy_from_slice(&data[first_part..]);
            indices.head = second_part;
        }

        Self::persist_indices(&mut mmap, &indices);
        mmap.flush_async()?;
        Ok(())
    }
}
