pub trait SizeDescription {
    /// Alignment in bytes
    const ALIGNMENT: usize;

    /// How many bytes would a field of this type take in a struct or a vec.
    /// Must be a multiple of alignment.
    const BYTE_SIZE: usize;

    /// To how many arguments would this struct "flatten" to for a function call.
    const NUM_ARGS: usize;

    type StructLayout;

    /// Layout of this struct in memory.
    /// 2*n is start n-th field, 2*n + 1 is end n-th field.
    /// 2*field_count is end on the entire struct.
    fn layout() -> Self::StructLayout;
}

type SimpleStructLayout = [usize; 3];

fn simple_layout(byte_size: usize) -> SimpleStructLayout {
    [0, byte_size, byte_size]
}

macro_rules! size_description_primitive {
    ($ty: ty, $bytes: literal) => {
        impl SizeDescription for $ty {
            const ALIGNMENT: usize = $bytes;
            const BYTE_SIZE: usize = $bytes;
            const NUM_ARGS: usize = 1;

            type StructLayout = SimpleStructLayout;

            #[inline]
            fn layout() -> Self::StructLayout {
                simple_layout(Self::BYTE_SIZE)
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
            const ALIGNMENT: usize = 4;
            const BYTE_SIZE: usize = 8;
            const NUM_ARGS: usize = 2;

            type StructLayout = SimpleStructLayout;

            #[inline]
            fn layout() -> Self::StructLayout {
                simple_layout(Self::BYTE_SIZE)
            }
        }
    };
}

size_description_fat_ptr_gen!(&[T]);
size_description_fat_ptr_gen!(Vec<T>);

macro_rules! size_description_fat_ptr {
    ($ty: ty) => {
        impl SizeDescription for $ty {
            const ALIGNMENT: usize = 4;
            const BYTE_SIZE: usize = 8;
            const NUM_ARGS: usize = 2;

            type StructLayout = SimpleStructLayout;

            #[inline]
            fn layout() -> Self::StructLayout {
                simple_layout(Self::BYTE_SIZE)
            }
        }
    };
}

size_description_fat_ptr!(&str);
size_description_fat_ptr!(String);

impl<T: SizeDescription> SizeDescription for &T {
    const ALIGNMENT: usize = T::ALIGNMENT;
    const BYTE_SIZE: usize = T::BYTE_SIZE;
    const NUM_ARGS: usize = T::NUM_ARGS;

    type StructLayout = T::StructLayout;

    fn layout() -> Self::StructLayout {
        T::layout()
    }
}

impl<T: SizeDescription> SizeDescription for Option<T> {
    const ALIGNMENT: usize = T::ALIGNMENT;
    const BYTE_SIZE: usize = Self::ALIGNMENT + T::BYTE_SIZE;
    const NUM_ARGS: usize = 1 + T::NUM_ARGS;

    type StructLayout = SimpleStructLayout;

    #[inline]
    fn layout() -> Self::StructLayout {
        simple_layout(Self::BYTE_SIZE)
    }
}

impl<T: SizeDescription, E: SizeDescription> SizeDescription for Result<T, E> {
    const ALIGNMENT: usize = usize_max(T::ALIGNMENT, E::ALIGNMENT);
    const BYTE_SIZE: usize = Self::ALIGNMENT + usize_max(T::BYTE_SIZE, E::BYTE_SIZE);
    const NUM_ARGS: usize = 1 + usize_max(T::NUM_ARGS, E::NUM_ARGS);

    type StructLayout = SimpleStructLayout;

    #[inline]
    fn layout() -> Self::StructLayout {
        simple_layout(Self::BYTE_SIZE)
    }
}

impl SizeDescription for () {
    const ALIGNMENT: usize = 1;
    const BYTE_SIZE: usize = 0;
    const NUM_ARGS: usize = 0;

    type StructLayout = [usize; 1];

    fn layout() -> Self::StructLayout {
        [0]
    }
}

impl<T: SizeDescription> SizeDescription for (T,) {
    const ALIGNMENT: usize = T::ALIGNMENT;
    const BYTE_SIZE: usize = T::BYTE_SIZE;
    const NUM_ARGS: usize = T::NUM_ARGS;

    type StructLayout = T::StructLayout;

    #[inline]
    fn layout() -> Self::StructLayout {
        T::layout()
    }
}

macro_rules! max_alignment {
    ($t1: ty, $t2: ty) => {
        usize_max(<$t1>::ALIGNMENT, <$t2>::ALIGNMENT)
    };
    ($t:ty, $($ts: ty),*) => {
        usize_max(<$t>::ALIGNMENT, max_alignment!($($ts),*))
    };
}

macro_rules! num_args {
    ($t1: ty, $t2: ty) => {
        <$t1>::NUM_ARGS + <$t2>::NUM_ARGS
    };
    ($t:ty, $($ts: ty),*) => {
        <$t>::NUM_ARGS + num_args!($($ts),*)
    };
}

impl<T0: SizeDescription, T1: SizeDescription> SizeDescription for (T0, T1) {
    const ALIGNMENT: usize = max_alignment!(T0, T1);
    const BYTE_SIZE: usize = next_multiple_of(
        next_multiple_of(T0::BYTE_SIZE, T1::ALIGNMENT) + T1::BYTE_SIZE,
        Self::ALIGNMENT,
    );
    const NUM_ARGS: usize = num_args!(T0, T1);

    type StructLayout = [usize; 5];

    #[inline]
    fn layout() -> Self::StructLayout {
        let start0 = 0;
        let end0 = start0 + T0::BYTE_SIZE;

        let start1 = next_multiple_of(end0, T1::ALIGNMENT);
        let end1 = start1 + T1::BYTE_SIZE;

        let start2 = next_multiple_of(end1, Self::ALIGNMENT);

        [start0, end0, start1, end1, start2]
    }
}

impl<T0: SizeDescription, T1: SizeDescription, T2: SizeDescription> SizeDescription
    for (T0, T1, T2)
{
    const ALIGNMENT: usize = max_alignment!(T0, T1, T2);
    const BYTE_SIZE: usize = next_multiple_of(
        next_multiple_of(
            next_multiple_of(T0::BYTE_SIZE, T1::ALIGNMENT) + T1::BYTE_SIZE,
            T2::ALIGNMENT,
        ) + T2::BYTE_SIZE,
        Self::ALIGNMENT,
    );
    const NUM_ARGS: usize = num_args!(T0, T1, T2);

    type StructLayout = [usize; 7];

    #[inline]
    fn layout() -> Self::StructLayout {
        let start0 = 0;
        let end0 = start0 + T0::BYTE_SIZE;

        let start1 = next_multiple_of(end0, T1::ALIGNMENT);
        let end1 = start1 + T1::BYTE_SIZE;

        let start2 = next_multiple_of(end1, T2::ALIGNMENT);
        let end2 = start2 + T2::BYTE_SIZE;

        let start3 = next_multiple_of(end2, Self::ALIGNMENT);

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
//     fn byte_size() -> usize {
//         Self::layout()[8]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::ALIGNMENT;
//         let start0 = 0;

//         let end0 = start0 + T0::BYTE_SIZE;
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::BYTE_SIZE;
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::BYTE_SIZE;
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::BYTE_SIZE;
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
//     fn byte_size() -> usize {
//         Self::layout()[10]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::ALIGNMENT;
//         let start0 = 0;

//         let end0 = start0 + T0::BYTE_SIZE;
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::BYTE_SIZE;
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::BYTE_SIZE;
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::BYTE_SIZE;
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::BYTE_SIZE;
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
//     fn byte_size() -> usize {
//         Self::layout()[12]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4, T5)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::ALIGNMENT;
//         let start0 = 0;

//         let end0 = start0 + T0::BYTE_SIZE;
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::BYTE_SIZE;
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::BYTE_SIZE;
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::BYTE_SIZE;
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::BYTE_SIZE;
//         let start5 = next_multiple_of(end4, align);

//         let end5 = start5 + T5::BYTE_SIZE;
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
//     fn byte_size() -> usize {
//         Self::layout()[14]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4, T5, T6)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::ALIGNMENT;
//         let start0 = 0;

//         let end0 = start0 + T0::BYTE_SIZE;
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::BYTE_SIZE;
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::BYTE_SIZE;
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::BYTE_SIZE;
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::BYTE_SIZE;
//         let start5 = next_multiple_of(end4, align);

//         let end5 = start5 + T5::BYTE_SIZE;
//         let start6 = next_multiple_of(end5, align);

//         let end6 = start5 + T5::BYTE_SIZE;
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
//     fn byte_size() -> usize {
//         Self::layout()[16]
//     }

//     #[inline]
//     fn num_args() -> usize {
//         num_args!(T0, T1, T2, T3, T4, T5, T6, T7)
//     }

//     #[inline]
//     fn layout() -> Self::StructLayout {
//         let align = Self::ALIGNMENT;
//         let start0 = 0;

//         let end0 = start0 + T0::BYTE_SIZE;
//         let start1 = next_multiple_of(end0, align);

//         let end1 = start1 + T1::BYTE_SIZE;
//         let start2 = next_multiple_of(end1, align);

//         let end2 = start2 + T2::BYTE_SIZE;
//         let start3 = next_multiple_of(end2, align);

//         let end3 = start3 + T3::BYTE_SIZE;
//         let start4 = next_multiple_of(end3, align);

//         let end4 = start4 + T4::BYTE_SIZE;
//         let start5 = next_multiple_of(end4, align);

//         let end5 = start5 + T5::BYTE_SIZE;
//         let start6 = next_multiple_of(end5, align);

//         let end6 = start5 + T5::BYTE_SIZE;
//         let start7 = next_multiple_of(end6, align);

//         let end7 = start6 + T6::BYTE_SIZE;
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

pub const fn usize_max(a: usize, b: usize) -> usize {
    if a >= b {
        a
    } else {
        b
    }
}

#[test]
fn test_next_multiple_of() {
    assert_eq!(next_multiple_of(5, 8), 8);
    assert_eq!(next_multiple_of(12, 8), 16);
    assert_eq!(next_multiple_of(12, 4), 12);
    assert_eq!(next_multiple_of(8, 8), 8);
}
