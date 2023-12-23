pub trait SizeDescription {
    type StructLayout;

    /// Alignment in bytes
    fn alignment() -> usize;

    /// How many bytes would a field of this type take in a struct or a vec.
    /// Must be a multiple of alignment.
    fn flat_byte_size() -> usize;

    /// How many "flatten" arguments would this create for the exported fn called
    fn num_args() -> usize;

    /// Layout of this struct in memory.
    /// 2*n is start n-th field, 2*n + 1 is end n-th field.
    /// 2*field_count is end on the entire struct.
    fn layout() -> Self::StructLayout;
}

type SimpleStructLayout = [usize; 3];

fn simple_layout(flat_byte_size: usize) -> SimpleStructLayout {
    [0, flat_byte_size, flat_byte_size]
}

macro_rules! size_description_primitive {
    ($ty: ty, $bytes: literal) => {
        impl SizeDescription for $ty {
            type StructLayout = SimpleStructLayout;

            fn alignment() -> usize {
                $bytes
            }

            fn flat_byte_size() -> usize {
                $bytes
            }

            fn num_args() -> usize {
                1
            }

            fn layout() -> Self::StructLayout {
                simple_layout(Self::flat_byte_size())
            }
        }
    };
}

size_description_primitive!(u8, 1);
size_description_primitive!(u16, 2);
size_description_primitive!(u32, 4);
size_description_primitive!(u64, 8);

size_description_primitive!(i8, 1);
size_description_primitive!(i16, 2);
size_description_primitive!(i32, 4);
size_description_primitive!(i64, 8);

size_description_primitive!(f32, 4);
size_description_primitive!(f64, 8);

size_description_primitive!(bool, 1);
size_description_primitive!(char, 4);

macro_rules! size_description_fat_ptr_gen {
    ($ty: ty) => {
        impl<T: SizeDescription> SizeDescription for $ty {
            type StructLayout = SimpleStructLayout;

            fn alignment() -> usize {
                4
            }

            fn flat_byte_size() -> usize {
                8
            }

            fn num_args() -> usize {
                2
            }

            fn layout() -> Self::StructLayout {
                simple_layout(Self::flat_byte_size())
            }
        }
    };
}

size_description_fat_ptr_gen!(&[T]);
size_description_fat_ptr_gen!(Vec<T>);

macro_rules! size_description_fat_ptr {
    ($ty: ty) => {
        impl SizeDescription for $ty {
            type StructLayout = SimpleStructLayout;

            fn alignment() -> usize {
                4
            }

            fn flat_byte_size() -> usize {
                8
            }

            fn num_args() -> usize {
                2
            }

            fn layout() -> Self::StructLayout {
                simple_layout(Self::flat_byte_size())
            }
        }
    };
}

size_description_fat_ptr!(&str);
size_description_fat_ptr!(String);

impl SizeDescription for () {
    type StructLayout = [usize; 1];

    fn alignment() -> usize {
        1
    }

    fn flat_byte_size() -> usize {
        0
    }

    fn num_args() -> usize {
        0
    }

    fn layout() -> Self::StructLayout {
        [0]
    }
}

impl<T: SizeDescription> SizeDescription for (T,) {
    type StructLayout = T::StructLayout;

    fn alignment() -> usize {
        T::alignment()
    }

    fn flat_byte_size() -> usize {
        T::flat_byte_size()
    }

    fn num_args() -> usize {
        T::num_args()
    }

    fn layout() -> Self::StructLayout {
        T::layout()
    }
}

macro_rules! max_alignment {
    ($($ty: ty),*) => {
        {
            let align = 1;
            $(let align = usize::max(align, <$ty>::alignment());)*
            align
        }
    }
}

impl<T: SizeDescription, U: SizeDescription> SizeDescription for (T, U) {
    type StructLayout = [usize; 5];

    fn alignment() -> usize {
        max_alignment!(T, U)
    }

    fn flat_byte_size() -> usize {
        Self::layout()[4]
    }

    fn num_args() -> usize {
        T::num_args() + U::num_args()
    }

    fn layout() -> Self::StructLayout {
        let align = Self::alignment();
        let start0 = 0;

        let end0 = start0 + T::flat_byte_size();
        let start1 = next_multiple_of(end0, align);

        let end1 = start1 + U::flat_byte_size();
        let start2 = next_multiple_of(end1, align);

        [start0, end0, start1, end1, start2]
    }
}

impl<T: SizeDescription, U: SizeDescription, V: SizeDescription> SizeDescription for (T, U, V) {
    type StructLayout = [usize; 7];

    fn alignment() -> usize {
        max_alignment!(T, U, V)
    }

    fn flat_byte_size() -> usize {
        Self::layout()[6]
    }

    fn num_args() -> usize {
        T::num_args() + U::num_args() + V::num_args()
    }

    fn layout() -> Self::StructLayout {
        let align = Self::alignment();
        let start = 0;

        let start0 = next_multiple_of(start, align);
        let end0 = start0 + T::flat_byte_size();

        let start1 = next_multiple_of(end0, align);
        let end1 = start1 + U::flat_byte_size();

        let start2 = next_multiple_of(end1, align);
        let end2 = start2 + V::flat_byte_size();

        let end = next_multiple_of(end2, align);

        [start0, end0, start1, end1, start2, end2, end]
    }
}

pub fn next_multiple_of(num: usize, multiple: usize) -> usize {
    ((num + multiple - 1) / multiple) * multiple
}

#[test]
fn test_next_multiple_of() {
    assert_eq!(next_multiple_of(5, 8), 8);
    assert_eq!(next_multiple_of(12, 8), 16);
    assert_eq!(next_multiple_of(12, 4), 12);
    assert_eq!(next_multiple_of(8, 8), 8);
}
