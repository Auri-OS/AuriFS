#![no_std]

extern crate alloc; 
use alloc::vec;
use alloc::vec::Vec;

pub const SECTOR_SIZE: usize = 512;

#[derive(Debug)]
pub enum DiskError {
    BufferTooSmall,
    OutOfBounds,
}

pub struct Ramdisk {
    storage: Vec<u8>,
    sector_size: usize,
}
impl Ramdisk {
    // créee un disque ram initialisé avec une taille donnée (en octets)
    pub fn new(sector_count: usize, sector_size: usize) -> Self {
        let total_bytes = sector_count * sector_size;
        Self {
            storage: vec![0; total_bytes],
            sector_size,
        }
    }


// lit le secteur 'sector_index' du disque ram et copie les données dans le tampon 'buffer'
pub fn read_sector(&self, sector_index: usize, buffer: &mut [u8]) -> Result<(), DiskError> {
    if buffer.len() < self.sector_size {
        return Err(DiskError::BufferTooSmall);
    }

    let start = sector_index * self.sector_size;
    let end = start + self.sector_size;

    if end > self.storage.len() {
        return Err(DiskError::OutOfBounds);
    }

    buffer[..self.sector_size].copy_from_slice(&self.storage[start..end]);
    Ok(())
}


// écrit le contenu du buffer dans le 'sector_index' du disque ram
pub fn write_sector(&mut self, sector_index: usize, buffer: &[u8]) -> Result<(), DiskError> {
    if buffer.len() < self.sector_size {
        return Err(DiskError::BufferTooSmall);
    }

    let start = sector_index * self.sector_size;
    let end = start + self.sector_size;

    if end > self.storage.len() {
        return Err(DiskError::OutOfBounds);
    }

    self.storage[start..end].copy_from_slice(&buffer[..self.sector_size]);
    Ok(())
}

pub fn sector_size(&self) -> usize {
    self.sector_size
}
}