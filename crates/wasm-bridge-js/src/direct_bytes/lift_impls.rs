use crate::conversions::FromJsValue;
use crate::Result;
use anyhow::{bail, Context};
use wasm_bindgen::JsValue;

use super::*;

macro_rules! lift_primitive {
    ($ty: ty) => {
        impl Lift for $ty {
            fn from_js_return<M: ReadableMemory>(value: &JsValue, _memory: &M) -> Result<Self> {
                Self::from_js_value(value)
            }

            fn from_js_args<M: ReadableMemory>(
                args: &mut JsArgsReader,
                _memory: &M,
            ) -> Result<Self> {
                Self::from_js_value(&args.next().context("Lift primitive with from_js_args")?)
            }

            fn read_from<M: ReadableMemory>(slice: &[u8], _memory: &M) -> Result<Self> {
                Ok(Self::from_le_bytes(slice.try_into()?))
            }
        }
    };
}

lift_primitive!(u8);
lift_primitive!(u16);
lift_primitive!(u32);
lift_primitive!(u64);

lift_primitive!(i8);
lift_primitive!(i16);
lift_primitive!(i32);
lift_primitive!(i64);

lift_primitive!(f32);
lift_primitive!(f64);

impl Lift for bool {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, _memory: &M) -> Result<Self> {
        let value = u8::from_js_value(value)?;
        u8_to_bool(value)
    }

    fn from_js_args<M: ReadableMemory>(args: &mut JsArgsReader, memory: &M) -> Result<Self> {
        Self::from_js_return(&args.next().context("Lift bool with from_js_args")?, memory)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], _memory: &M) -> Result<Self> {
        let value = slice[0];
        u8_to_bool(value)
    }
}

fn u8_to_bool(value: u8) -> Result<bool> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        n => bail!("Invalid boolean value: {n}"),
    }
}

impl Lift for char {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> Result<Self> {
        let code = u32::from_js_return(value, memory)?;
        char::from_u32(code).context("Invalid character bytes")
    }

    fn from_js_args<M: ReadableMemory>(args: &mut JsArgsReader, memory: &M) -> Result<Self> {
        Self::from_js_return(&args.next().context("Lift char with from_js_args")?, memory)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> Result<Self> {
        let code = u32::read_from(slice, memory)?;
        char::from_u32(code).context("Invalid character bytes")
    }
}

impl<T: Lift> Lift for Vec<T> {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> Result<Self> {
        let addr = u32::from_js_value(value)? as usize;

        let mut addr_and_len = [0u8; 8];
        memory.read_to_slice(addr, &mut addr_and_len);

        Self::read_from(&addr_and_len, memory)
    }

    fn from_js_args<M: ReadableMemory>(args: &mut JsArgsReader, memory: &M) -> Result<Self> {
        let addr = args.next().context("Get addr in from_js_args for Vec")?;
        let len = args.next().context("Get len in from_js_args for Vec")?;

        let addr = u32::from_js_value(&addr)? as usize;
        let len = u32::from_js_value(&len)? as usize;

        read_vec_from(addr, len, memory)
    }

    fn read_from<M: ReadableMemory>(addr_and_len: &[u8], memory: &M) -> Result<Self> {
        if addr_and_len.len() != 8 {
            bail!(
                "Lift vec: addr_and_len have length {} instead of 8",
                addr_and_len.len()
            );
        }

        let addr = u32::from_le_bytes(addr_and_len[0..4].try_into().unwrap()) as usize;
        let len = u32::from_le_bytes(addr_and_len[4..8].try_into().unwrap()) as usize;

        read_vec_from(addr, len, memory)
    }
}

fn read_vec_from<T: Lift, M: ReadableMemory>(
    addr: usize,
    len: usize,
    memory: &M,
) -> Result<Vec<T>> {
    let size = T::BYTE_SIZE;
    let data = memory.read_to_vec(addr, size * len);

    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        result.push(T::read_from(&data[i * size..(i + 1) * size], memory)?);
    }
    Ok(result)
}

impl Lift for String {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> Result<Self> {
        let bytes = Vec::from_js_return(value, memory)?;
        Ok(String::from_utf8(bytes)?)
    }

    fn from_js_args<M: ReadableMemory>(args: &mut JsArgsReader, memory: &M) -> Result<Self> {
        let bytes = Vec::from_js_args(args, memory)?;
        Ok(String::from_utf8(bytes)?)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> Result<Self> {
        let bytes = Vec::read_from(slice, memory)?;
        Ok(String::from_utf8(bytes)?)
    }
}

impl<T: Lift> Lift for Option<T> {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> anyhow::Result<Self> {
        Self::from_js_ptr_return(value, memory)
    }

    fn from_js_args<M: ReadableMemory>(
        args: &mut JsArgsReader,
        memory: &M,
    ) -> anyhow::Result<Self> {
        let variant = args.next().context("Get option variant tag")?;
        let variant = u8::from_js_value(&variant)?;
        match variant {
            0 => {
                for _ in 0..T::NUM_ARGS {
                    args.next().context("Skipping unused option::none args")?;
                }
                Ok(Self::None)
            }
            1 => Ok(Self::Some(T::from_js_args(args, memory)?)),
            other => bail!("Invalid option variant tag: {other}"),
        }
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> anyhow::Result<Self> {
        let variant = slice[0];
        match variant {
            0 => Ok(Self::None),
            1 => Ok(Some(T::read_from(&slice[(Self::ALIGNMENT)..], memory)?)),
            other => bail!("Invalid option variant tag: {other}"),
        }
    }
}

impl<T: Lift, E: Lift> Lift for Result<T, E> {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> anyhow::Result<Self> {
        if Self::NUM_ARGS == 1 {
            let variant = u8::from_js_value(&value)?;
            match variant {
                // TODO: The (T/E)::from_js_return are not really needed,
                // since we know both T and E are the unit.
                0 => Ok(Self::Ok(T::from_js_return(value, memory)?)),
                1 => Ok(Self::Err(E::from_js_return(value, memory)?)),
                other => bail!("Invalid result variant tag: {other}"),
            }
        } else {
            Self::from_js_ptr_return(value, memory)
        }
    }

    fn from_js_args<M: ReadableMemory>(
        args: &mut JsArgsReader,
        memory: &M,
    ) -> anyhow::Result<Self> {
        let variant = args.next().context("Get result variant tag")?;
        let variant = u8::from_js_value(&variant)?;

        let (result, args_read) = match variant {
            0 => (Self::Ok(T::from_js_args(args, memory)?), T::NUM_ARGS),
            1 => (Self::Err(E::from_js_args(args, memory)?), E::NUM_ARGS),
            other => bail!("Invalid result variant tag: {other}"),
        };

        // Start from 1 to account for the initial variant tag
        for _ in 1..(Self::NUM_ARGS - args_read) {
            args.next().context("Skipping unused result args")?;
        }

        Ok(result)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> anyhow::Result<Self> {
        let variant = slice[0];
        match variant {
            0 => Ok(Self::Ok(T::read_from(
                &slice[(Self::ALIGNMENT)..(Self::ALIGNMENT + T::BYTE_SIZE)],
                memory,
            )?)),
            1 => Ok(Self::Err(E::read_from(
                &slice[(Self::ALIGNMENT)..(Self::ALIGNMENT + E::BYTE_SIZE)],
                memory,
            )?)),
            other => bail!("Invalid result variant tag: {other}"),
        }
    }
}

impl Lift for () {
    fn from_js_return<M: ReadableMemory>(_val: &JsValue, _memory: &M) -> Result<Self> {
        Ok(())
    }

    fn from_js_args<M: ReadableMemory>(_args: &mut JsArgsReader, _memory: &M) -> Result<Self> {
        Ok(())
    }

    fn read_from<M: ReadableMemory>(_slice: &[u8], _memory: &M) -> Result<Self> {
        Ok(())
    }
}

impl<T: Lift> Lift for (T,) {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> Result<Self> {
        Ok((T::from_js_return(value, memory)?,))
    }

    fn from_js_args<M: ReadableMemory>(args: &mut JsArgsReader, memory: &M) -> Result<Self> {
        Ok((T::from_js_args(args, memory)?,))
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> Result<Self> {
        Ok((T::read_from(slice, memory)?,))
    }
}

macro_rules! lift_tuple {
    ($(($name: ident, $start: literal, $end: literal)),*) => {
        impl<$($name: Lift),*> Lift for ($($name),*) {
            fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> Result<Self> {
                Self::from_js_ptr_return(value, memory)
            }

            fn from_js_args<M: ReadableMemory>(args: &mut JsArgsReader, memory: &M) -> Result<Self> {
                Ok((
                    $(<$name>::from_js_args(args, memory)?),*
                ))
            }

            fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> Result<Self> {
                let layout = Self::layout();
                Ok((
                    $(<$name>::read_from(&slice[layout[$start]..layout[$end]], memory)?),*
                ))
            }
        }
    };
}

#[rustfmt::skip]
lift_tuple!((T0, 0, 1), (T1, 2, 3));
#[rustfmt::skip]
lift_tuple!((T0, 0, 1), (T1, 2, 3), (T2, 4, 5));
#[rustfmt::skip]
lift_tuple!((T0, 0, 1), (T1, 2, 3), (T2, 4, 5), (T3, 6, 7));
#[rustfmt::skip]
lift_tuple!((T0, 0, 1), (T1, 2, 3), (T2, 4, 5), (T3, 6, 7), (T4, 8, 9));
#[rustfmt::skip]
lift_tuple!((T0, 0, 1), (T1, 2, 3), (T2, 4, 5), (T3, 6, 7), (T4, 8, 9), (T5, 10, 11));
#[rustfmt::skip]
lift_tuple!((T0, 0, 1), (T1, 2, 3), (T2, 4, 5), (T3, 6, 7), (T4, 8, 9), (T5, 10, 11), (T6, 12, 13));
#[rustfmt::skip]
lift_tuple!((T0, 0, 1), (T1, 2, 3), (T2, 4, 5), (T3, 6, 7), (T4, 8, 9), (T5, 10, 11), (T6, 12, 13), (T7, 14, 15));
