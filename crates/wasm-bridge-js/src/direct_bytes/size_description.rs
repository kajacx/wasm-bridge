pub trait SizeDescription {
    /// Alignment in bytes
    const ALIGNMENT: usize;

    /// How many bytes would a field of this type take in a struct or a vec.
    /// Must be a multiple of alignment.
    const FLAT_BYTE_SIZE: usize;

    type StructLayout;

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
            const FLAT_BYTE_SIZE: usize = $bytes;
            const ALIGNMENT: usize = $bytes;

            fn num_args() -> usize {
                1
            }

            fn layout() -> Self::StructLayout {
                simple_layout(Self::FLAT_BYTE_SIZE)
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
            const FLAT_BYTE_SIZE: usize = 8;
            const ALIGNMENT: usize = 4;

            fn num_args() -> usize {
                2
            }

            fn layout() -> Self::StructLayout {
                simple_layout(Self::FLAT_BYTE_SIZE)
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
            const FLAT_BYTE_SIZE: usize = 8;
            const ALIGNMENT: usize = 4;



            fn num_args() -> usize {
                2
            }

            fn layout() -> Self::StructLayout {
                simple_layout(Self::FLAT_BYTE_SIZE)
            }
        }
    };
}

size_description_fat_ptr!(&str);
size_description_fat_ptr!(String);

impl<T: SizeDescription> SizeDescription for Option<T> {
    type StructLayout = SimpleStructLayout;
    const FLAT_BYTE_SIZE: usize = Self::ALIGNMENT + T::FLAT_BYTE_SIZE ;
    const ALIGNMENT: usize = T::ALIGNMENT;


    #[inline]
    fn num_args() -> usize {
        1 + T::num_args()
    }

    #[inline]
    fn layout() -> Self::StructLayout {
        simple_layout(Self::FLAT_BYTE_SIZE)
    }
}

impl<T: SizeDescription, E: SizeDescription> SizeDescription for Result<T, E> {
    type StructLayout = SimpleStructLayout;
    const FLAT_BYTE_SIZE: usize = Self::ALIGNMENT + usize::max(T::FLAT_BYTE_SIZE, E::FLAT_BYTE_SIZE);
    const ALIGNMENT: usize = usize::max(T::ALIGNMENT, E::ALIGNMENT);


    #[inline]
    fn num_args() -> usize {
        1 + usize::max(T::num_args(), E::num_args())
    }

    #[inline]
    fn layout() -> Self::StructLayout {
        simple_layout(Self::flat_byte_size())
    }
}

impl SizeDescription for () {
    type StructLayout = [usize; 1];
    const ALIGNMENT: usize = 1;
    const FLAT_BYTE_SIZE: usize = 0;


    fn num_args() -> usize {
        0
    }

    fn layout() -> Self::StructLayout {
        [0]
    }
}

impl<T: SizeDescription> SizeDescription for (T,) {
    type StructLayout = T::StructLayout;
    const ALIGNMENT: usize = Self::Alignment;
    const FLAT_BYTE_SIZE: usize = Self::FLAT_BYTE_SIZE;

    #[inline]
    fn num_args() -> usize {
        T::num_args()
    }

    #[inline]
    fn layout() -> Self::StructLayout {
        T::layout()
    }
}

macro_rules! max_alignment {
    ($t1: ty, $t2: ty) => {
        usize::max($t1::ALIGNMENT, $t2::ALIGNMENT)
    }
    ($t:ty, $($ts: ty),*) => {
        usize::max($t::ALIGNMENT, max_alignment!($($ts),*))
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

impl<T0: SizeDescription, T1: SizeDescription> SizeDescription for (T0, T1) {
    type StructLayout = [usize; 5];
    const ALIGNMENT: usize = max_alignment!(T0, T1);
    const FLAT_BYTE_SIZE: usize = next_multiple_of(T0::FLAT_BYTE_SIZE, Self::ALIGNMENT) + next_multiple_of(T1::FLAT_BYTE_SIZE, Self::ALIGNMENT);

    #[inline]
    fn num_args() -> usize {
        num_args!(T0, T1)
    }

    #[inline]
    fn layout() -> Self::StructLayout {
        let align = Self::alignment();
        let start0 = 0;

        let end0 = start0 + T0::flat_byte_size();
        let start1 = next_multiple_of(end0, align);

        let end1 = start1 + T1::flat_byte_size();
        let start2 = next_multiple_of(end1, align);

        [start0, end0, start1, end1, start2]
    }
}

impl<T0: SizeDescription, T1: SizeDescription, T2: SizeDescription> SizeDescription
    for (T0, T1, T2)
{
    type StructLayout = [usize; 7];
    const ALIGNMENT: usize = max_alignment!(T0, T1, T2);
    // FIXME: this is wrong
    const FLAT_BYTE_SIZE: usize = next_multiple_of(T0::FLAT_BYTE_SIZE, Self::ALIGNMENT) 
    + next_multiple_of(T1::FLAT_BYTE_SIZE, Self::ALIGNMENT)
    + next_multiple_of(T2::FLAT_BYTE_SIZE, Self::ALIGNMENT);

    #[inline]
    fn num_args() -> usize {
        num_args!(T0, T1, T2)
    }

    #[inline]
    fn layout() -> Self::StructLayout {
        let align = Self::alignment();
        let start0 = 0;

        let end0 = start0 + T0::flat_byte_size();
        let start1 = next_multiple_of(end0, align);

        let end1 = start1 + T1::flat_byte_size();
        let start2 = next_multiple_of(end1, align);

        let end2 = start2 + T2::flat_byte_size();
        let start3 = next_multiple_of(end2, align);

        [start0, end0, start1, end1, start2, end2, start3]
    }
}

// impl<T0: SizeDescription, T1: SizeDescription, T2: SizeDescription, T3: SizeDescription>
//     SizeDescription for (T0, T1, T2, T3)
// {
//     type StructLayout = [usize; 9];

//     #[inline]
//     fn alignment() -> usize {
//         max_alignment!(T0, T1, T2, T3)
//     }

//     #[inline]
//     fn flat_byte_size() -> usize {
//         Self::layout()[8]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::alignment();
//         let start0 = 0;

//         let end0 = start0 + T0::flat_byte_size();
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::flat_byte_size();
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::flat_byte_size();
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::flat_byte_size();
//         let start4 = next_multiple_of(end3, align);

//         [
//             start0, end0, start1, end1, start2, end2, start3, end3, start4,
//         ]
//     }
// }

// impl<
//         T0: SizeDescription,
//         T1: SizeDescription,
//         T2: SizeDescription,
//         T3: SizeDescription,
//         T4: SizeDescription,
//     > SizeDescription for (T0, T1, T2, T3, T4)
// {
//     type StructLayout = [usize; 11];

//     #[inline]
//     fn alignment() -> usize {
//         max_alignment!(T0, T1, T2, T3, T4)
//     }

//     #[inline]
//     fn flat_byte_size() -> usize {
//         Self::layout()[10]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::alignment();
//         let start0 = 0;

//         let end0 = start0 + T0::flat_byte_size();
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::flat_byte_size();
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::flat_byte_size();
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::flat_byte_size();
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::flat_byte_size();
//         let start5 = next_multiple_of(end4, align);

//         [
//             start0, end0, start1, end1, start2, end2, start3, end3, start4, end4, start5,
//         ]
//     }
// }

// impl<
//         T0: SizeDescription,
//         T1: SizeDescription,
//         T2: SizeDescription,
//         T3: SizeDescription,
//         T4: SizeDescription,
//         T5: SizeDescription,
//     > SizeDescription for (T0, T1, T2, T3, T4, T5)
// {
//     type StructLayout = [usize; 13];

//     #[inline]
//     fn alignment() -> usize {
//         max_alignment!(T0, T1, T2, T3, T4, T5)
//     }

//     #[inline]
//     fn flat_byte_size() -> usize {
//         Self::layout()[12]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4, T5)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::alignment();
//         let start0 = 0;

//         let end0 = start0 + T0::flat_byte_size();
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::flat_byte_size();
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::flat_byte_size();
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::flat_byte_size();
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::flat_byte_size();
//         let start5 = next_multiple_of(end4, align);

//         let end5 = start5 + T5::flat_byte_size();
//         let start6 = next_multiple_of(end5, align);

//         [
//             start0, end0, start1, end1, start2, end2, start3, end3, start4, end4, start5, end5,
//             start6,
//         ]
//     }
// }

// impl<
//         T0: SizeDescription,
//         T1: SizeDescription,
//         T2: SizeDescription,
//         T3: SizeDescription,
//         T4: SizeDescription,
//         T5: SizeDescription,
//         T6: SizeDescription,
//     > SizeDescription for (T0, T1, T2, T3, T4, T5, T6)
// {
//     type StructLayout = [usize; 15];

//     #[inline]
//     fn alignment() -> usize {
//         max_alignment!(T0, T1, T2, T3, T4, T5, T6)
//     }

//     #[inline]
//     fn flat_byte_size() -> usize {
//         Self::layout()[14]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4, T5, T6)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::alignment();
//         let start0 = 0;

//         let end0 = start0 + T0::flat_byte_size();
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::flat_byte_size();
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::flat_byte_size();
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::flat_byte_size();
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::flat_byte_size();
//         let start5 = next_multiple_of(end4, align);

//         let end5 = start5 + T5::flat_byte_size();
//         let start6 = next_multiple_of(end5, align);

//         let end6 = start5 + T5::flat_byte_size();
//         let start7 = next_multiple_of(end6, align);

//         [
//             start0, end0, start1, end1, start2, end2, start3, end3, start4, end4, start5, end5,
//             start6, end6, start7,
//         ]
//     }
// }

// impl<
//         T0: SizeDescription,
//         T1: SizeDescription,
//         T2: SizeDescription,
//         T3: SizeDescription,
//         T4: SizeDescription,
//         T5: SizeDescription,
//         T6: SizeDescription,
//         T7: SizeDescription,
//     > SizeDescription for (T0, T1, T2, T3, T4, T5, T6, T7)
// {
//     type StructLayout = [usize; 17];

//     #[inline]
//     fn alignment() -> usize {
//         max_alignment!(T0, T1, T2, T3, T4, T5, T6, T7)
//     }

//     #[inline]
//     fn flat_byte_size() -> usize {
//         Self::layout()[16]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4, T5, T6, T7)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::alignment();
//         let start0 = 0;

//         let end0 = start0 + T0::flat_byte_size();
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::flat_byte_size();
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::flat_byte_size();
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::flat_byte_size();
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::flat_byte_size();
//         let start5 = next_multiple_of(end4, align);

//         let end5 = start5 + T5::flat_byte_size();
//         let start6 = next_multiple_of(end5, align);

//         let end6 = start5 + T5::flat_byte_size();
//         let start7 = next_multiple_of(end6, align);

//         let end7 = start6 + T6::flat_byte_size();
//         let start8 = next_multiple_of(end7, align);

//         [
//             start0, end0, start1, end1, start2, end2, start3, end3, start4, end4, start5, end5,
//             start6, end6, start7, end7, start8,
//         ]
//     }
// }

pub const fn next_multiple_of(num: usize, multiple: usize) -> usize {
    ((num + multiple - 1) / multiple) * multiple
}

#[test]
fn test_next_multiple_of() {
    assert_eq!(next_multiple_of(5, 8), 8);
    assert_eq!(next_multiple_of(12, 8), 16);
    assert_eq!(next_multiple_of(12, 4), 12);
    assert_eq!(next_multiple_of(8, 8), 8);
}
