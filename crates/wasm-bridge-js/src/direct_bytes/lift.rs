use anyhow::Result;

use super::SizeDescription;

pub trait Lift: SizeDescription + Sized {
    fn read_from<T: ReadableMemory>(memory: T, addr: usize) -> Result<Self>;
}

pub trait ReadableMemory {
    fn read_to_slice(&self, addr: usize, target: &mut [u8]);

    fn read_to_vec(&self, addr: usize, len: usize) -> Vec<u8> {
        // TODO: could do uninit memory with unsafe code
        let mut vec: Vec<u8> = (0..len).map(|_| 0).collect();
        self.read_slice(addr, &mut vec);
        vec
    }
}
