mod lift;
mod lift_impls;
mod lower;
mod lower_impls;

pub use lift::*;
pub use lower::*;

pub trait SizeDescription {
    /// Alignment in bytes
    fn alignment() -> usize;

    /// How many bytes would a field of this type take in a struct
    fn flat_byte_size() -> usize;
}

impl SizeDescription for i32 {
    fn alignment() -> usize {
        4
    }

    fn flat_byte_size() -> usize {
        4
    }
}

impl SizeDescription for u32 {
    fn alignment() -> usize {
        4
    }

    fn flat_byte_size() -> usize {
        4
    }
}

impl<T: SizeDescription> SizeDescription for &[T] {
    fn alignment() -> usize {
        4
    }

    fn flat_byte_size() -> usize {
        8
    }
}

impl<T: SizeDescription> SizeDescription for Vec<T> {
    fn alignment() -> usize {
        4
    }

    fn flat_byte_size() -> usize {
        8
    }
}
