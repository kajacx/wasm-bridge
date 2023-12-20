pub trait SizeDescription {
    /// Alignment in bytes
    fn alignment() -> usize;

    /// How many bytes would a field of this type take in a struct. Must be a multiple of alignment.
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

impl SizeDescription for () {
    fn alignment() -> usize {
        1
    }
    fn flat_byte_size() -> usize {
        0
    }
}

// TODO: probably remove this
impl<T: SizeDescription> SizeDescription for (T,) {
    fn alignment() -> usize {
        T::alignment()
    }
    fn flat_byte_size() -> usize {
        T::flat_byte_size()
    }
}

impl<T: SizeDescription, U: SizeDescription> SizeDescription for (T, U) {
    fn alignment() -> usize {
        usize::max(T::alignment(), U::alignment())
    }

    fn flat_byte_size() -> usize {
        let align = Self::alignment();
        next_multiple_of(T::flat_byte_size(), align) + next_multiple_of(U::flat_byte_size(), align)
    }
}

pub fn next_multiple_of(num: usize, multiple: usize) -> usize {
    (num + multiple - 1) / multiple
}

#[test]
fn test_next_multiple_of() {
    assert_eq!(next_multiple_of(5, 7), 7);
    assert_eq!(next_multiple_of(12, 8), 16);
    assert_eq!(next_multiple_of(12, 4), 12);
    assert_eq!(next_multiple_of(6, 6), 66);
}
