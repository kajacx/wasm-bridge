use crate::component::{Resource, ResourceAny};

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

macro_rules! size_description_fat_ptr {
    ([$($name: ident),*], $ty: ty) => {
        impl<$($name: SizeDescription),*> SizeDescription for $ty {
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

size_description_fat_ptr!([T], &[T]);
size_description_fat_ptr!([T], Vec<T>);
size_description_fat_ptr!([], &str);
size_description_fat_ptr!([], String);

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

impl<T> SizeDescription for Resource<T> {
    const ALIGNMENT: usize = 4;
    const BYTE_SIZE: usize = 4;
    const NUM_ARGS: usize = 1;

    type StructLayout = SimpleStructLayout;

    #[inline]
    fn layout() -> Self::StructLayout {
        simple_layout(Self::BYTE_SIZE)
    }
}

impl SizeDescription for ResourceAny {
    const ALIGNMENT: usize = 4;
    const BYTE_SIZE: usize = 4;
    const NUM_ARGS: usize = 1;

    type StructLayout = SimpleStructLayout;

    #[inline]
    fn layout() -> Self::StructLayout {
        simple_layout(Self::BYTE_SIZE)
    }
}

impl SizeDescription for anyhow::Error {
    const ALIGNMENT: usize = 4;
    const BYTE_SIZE: usize = 4;
    const NUM_ARGS: usize = 1;

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

wasm_bridge_macros::size_description_tuple!(2);
wasm_bridge_macros::size_description_tuple!(3);
wasm_bridge_macros::size_description_tuple!(4);
wasm_bridge_macros::size_description_tuple!(5);
wasm_bridge_macros::size_description_tuple!(6);
wasm_bridge_macros::size_description_tuple!(7);
wasm_bridge_macros::size_description_tuple!(8);

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
