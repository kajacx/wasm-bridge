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

impl<T: SizeDescription> SizeDescription for Option<T> {
    type StructLayout = SimpleStructLayout;

    fn alignment() -> usize {
        T::alignment()
    }

    fn flat_byte_size() -> usize {
        T::flat_byte_size() + Self::alignment()
    }

    fn num_args() -> usize {
        T::num_args() + 1
    }

    fn layout() -> Self::StructLayout {
        simple_layout(Self::flat_byte_size())
    }
}

impl<T: SizeDescription, E: SizeDescription> SizeDescription for Result<T, E> {
    type StructLayout = SimpleStructLayout;

    fn alignment() -> usize {
        usize::max(T::alignment(), E::alignment())
    }

    fn flat_byte_size() -> usize {
        usize::max(T::flat_byte_size(), E::flat_byte_size()) + Self::alignment()
    }

    fn num_args() -> usize {
        usize::max(T::num_args(), E::num_args()) + 1
    }

    fn layout() -> Self::StructLayout {
        simple_layout(Self::flat_byte_size())
    }
}

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

macro_rules! num_args {
    ($($ty: ty),*) => {
        {
            let args = 0;
            $(let args = args + <$ty>::num_args();)*
            args
        }
    };
}

macro_rules! layout_impl {
    ("stop") => { };
    (0, $($n: tt),*) => {
        let e0 = s0 + T0::flat_byte_size();
        let s1 = next_multiple_of(e0, align);
        layout_impl!($($n),*)
    };
    (1, $($n: tt),*) => {
        let e1 = s1 + T1::flat_byte_size();
        let s2 = next_multiple_of(e1, align);
        layout_impl!($($n),*)
    };
    (2, $($n: tt),*) => {
        let e2 = s2 + T2::flat_byte_size();
        let s3 = next_multiple_of(e2, align);
        layout_impl!($($n),*)
    };
    (3, $($n: tt),*) => {
        let e3 = s3 + T3::flat_byte_size();
        let s4 = next_multiple_of(e3, align);
        layout_impl!($($n),*)
    };
}

macro_rules! size_description_tuple {
    (($($name: ident),*), $len: literal, ($($layout: tt),*), $ret: expr) => {
        impl<$($name: SizeDescription),*> SizeDescription for ($($name),*) {
            type StructLayout = [usize; 2 * $len + 1];

            fn alignment() -> usize {
                max_alignment!($($name),*)
            }

            fn flat_byte_size() -> usize {
                Self::layout[$len * 2]
            }

            fn num_args() -> usize {
                num_args!($($name),*)
            }

            fn layout() -> Self::StructLayout {
                let align = Self::alignment();
                let s0 = 0;

                layout_impl!($($layout),*);

                $ret
            }
        }
    };
}

size_description_tuple!((T0, T1), 2, (0, 1, "stop"), [s0, e0, s1, e1, s2]);

// impl<T: SizeDescription, U: SizeDescription> SizeDescription for (T, U) {
//     type StructLayout = [usize; 5];

//     fn alignment() -> usize {
//         max_alignment!(T, U)
//     }

//     fn flat_byte_size() -> usize {
//         Self::layout()[4]
//     }

//     fn num_args() -> usize {
//         num_args!(T, U)
//     }

//     fn layout() -> Self::StructLayout {
//         let align = Self::alignment();
//         let start0 = 0;

//         let end0 = start0 + T::flat_byte_size();
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + U::flat_byte_size();
//         let start2 = next_multiple_of(end1, align);

//         [start0, end0, start1, end1, start2]
//     }
// }

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
