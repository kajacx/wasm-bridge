use super::*;
use crate::component::{Resource, ResourceAny};
use crate::Result;
use crate::ToJsValue;
use wasm_bindgen::JsValue;

macro_rules! lower_primitive {
    ($ty: ty) => {
        impl Lower for $ty {
            fn to_js_args<M: WriteableMemory>(
                &self,
                args: &mut JsArgsWriter,
                _memory: &M,
            ) -> Result<()> {
                args.push(&self.to_js_value());
                Ok(())
            }

            fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<JsValue> {
                Ok(self.to_js_value())
            }

            fn write_to<M: WriteableMemory>(
                &self,
                buffer: &mut ByteBuffer,
                _memory: &M,
            ) -> Result<()> {
                buffer.write_bytes(&self.to_le_bytes());
                Ok(())
            }
        }
    };
}

lower_primitive!(u8);
lower_primitive!(u16);
lower_primitive!(u32);
lower_primitive!(u64);

lower_primitive!(i8);
lower_primitive!(i16);
lower_primitive!(i32);
lower_primitive!(i64);

lower_primitive!(f32);
lower_primitive!(f64);

impl Lower for bool {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        (*self as u8).to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        (*self as u8).to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        buffer.write(&(*self as u8), memory)
    }
}

impl Lower for char {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        (*self as u32).to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        (*self as u32).to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        buffer.write(&(*self as u32), memory)
    }
}

impl<T: Lower> Lower for &[T] {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        let addr = write_vec_data(self, memory)? as u32;
        let len = self.len() as u32;

        // First address, then element count
        args.push(&addr.into());
        args.push(&len.into());

        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        let mut buffer = memory.allocate(T::ALIGNMENT, T::BYTE_SIZE * self.len())?;
        self.write_to(&mut buffer, memory)?;

        let addr = memory.flush(buffer) as u32;
        Ok(addr.to_js_value())
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        let addr = write_vec_data(self, memory)? as u32;
        let len = self.len() as u32;

        buffer.write(&addr, memory)?;
        buffer.write(&len, memory)?;

        Ok(())
    }
}

impl<T: Lower> Lower for Vec<T> {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        self.as_slice().to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.as_slice().to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.as_slice().write_to(buffer, memory)
    }
}

impl Lower for &str {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        self.as_bytes().to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.as_bytes().to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.as_bytes().write_to(buffer, memory)
    }
}

impl Lower for String {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        self.as_bytes().to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.as_bytes().to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.as_bytes().write_to(buffer, memory)
    }
}

// Writes the data to the memory, returning the starting address of the data
fn write_vec_data<T: Lower, M: WriteableMemory>(data: &[T], memory: &M) -> Result<usize> {
    // Allocate space for all the elements
    let mut buffer = memory.allocate(T::ALIGNMENT, T::BYTE_SIZE * data.len())?;

    // Then write the elements to the slice buffer
    for elem in data {
        elem.write_to(&mut buffer, memory)?;
    }

    // Then actually write the slice buffer to memory and return the address
    Ok(memory.flush(buffer))
}

impl<T: Lower> Lower for &T {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        T::to_js_args(self, args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        T::to_js_return(self, memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        T::write_to(self, buffer, memory)
    }
}

impl<T: Lower> Lower for Option<T> {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        match self {
            Some(value) => {
                args.push(&1u8.to_js_value());
                value.to_js_args(args, memory)?;
            }
            None => {
                args.push(&0u8.to_js_value());
                args.skip(T::NUM_ARGS);
            }
        };
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.to_js_ptr_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        match self {
            Some(value) => {
                buffer.write(&1u8, memory)?;
                buffer.skip(Self::ALIGNMENT - 1);

                buffer.write(value, memory)?;
            }
            None => {
                buffer.write(&0u8, memory)?;
                buffer.skip(Self::BYTE_SIZE - 1);
            }
        }
        Ok(())
    }
}

impl<T: Lower, E: Lower> Lower for Result<T, E> {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        let args_written = match self {
            Ok(value) => {
                args.push(&0u8.to_js_value());
                value.to_js_args(args, memory)?;
                T::NUM_ARGS
            }
            Err(error) => {
                args.push(&1u8.to_js_value());
                error.to_js_args(args, memory)?;
                E::NUM_ARGS
            }
        };

        // Subtract an extra 1 to account for the initial variant tag
        args.skip(Self::NUM_ARGS - args_written - 1);

        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        if Self::NUM_ARGS == 1 {
            match self {
                Ok(_) => Ok(0u8.to_js_value()),
                Err(_) => Ok(1u8.to_js_value()),
            }
        } else {
            self.to_js_ptr_return(memory)
        }
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        let bytes_written = match self {
            Ok(value) => {
                buffer.write(&0u8, memory)?;
                buffer.skip(Self::ALIGNMENT - 1);

                value.write_to(buffer, memory)?;
                T::BYTE_SIZE
            }
            Err(error) => {
                buffer.write(&1u8, memory)?;
                buffer.skip(Self::ALIGNMENT - 1);

                error.write_to(buffer, memory)?;
                E::BYTE_SIZE
            }
        };

        // Variant tag takes 1 whole alignment
        buffer.skip(Self::BYTE_SIZE - bytes_written - Self::ALIGNMENT);

        Ok(())
    }
}

impl<T> Lower for Resource<T> {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        args.push(&self.to_js_return(memory)?);
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<JsValue> {
        Ok(self.rep().to_js_value())
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        buffer.write(&self.rep(), memory)
    }
}

impl Lower for ResourceAny {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        args.push(&self.to_js_return(memory)?);
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<JsValue> {
        Ok(self.id.to_js_value())
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        buffer.write(&self.id, memory)
    }
}

// Kind of hacky, but it's needed for deriving LowerJs for StreamError
impl Lower for anyhow::Error {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        args.push(&self.to_js_return(memory)?);
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<JsValue> {
        Ok(0.to_js_value())
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        buffer.write(&0, memory)
    }
}

impl Lower for () {
    fn to_js_args<M: WriteableMemory>(&self, _args: &mut JsArgsWriter, _memory: &M) -> Result<()> {
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<JsValue> {
        Ok(JsValue::UNDEFINED)
    }

    fn write_to<M: WriteableMemory>(&self, _buffer: &mut ByteBuffer, _memory: &M) -> Result<()> {
        Ok(())
    }
}

impl<T: Lower> Lower for (T,) {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
        self.0.to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.0.to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.0.write_to(buffer, memory)
    }
}

macro_rules! lower_tuple {
    ($(($name: ident, $index: tt, $next: literal, $end: literal)),*) => {
        impl<$($name: Lower),*> Lower for ($($name),*) {
            fn to_js_args<M: WriteableMemory>(&self, args: &mut JsArgsWriter, memory: &M) -> Result<()> {
                $(self.$index.to_js_args(args, memory)?;)*
                Ok(())
            }

            fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
                self.to_js_ptr_return(memory)
            }

            fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
                let layout = Self::layout();
                $(self.$index.write_to(buffer, memory)?;)*
                $(buffer.skip(layout[$next] - layout[$end]);)*
                Ok(())
            }
        }
    };
}

#[rustfmt::skip]
lower_tuple!((T0, 0, 2, 1), (T1, 1, 4, 3));
#[rustfmt::skip]
lower_tuple!((T0, 0, 2, 1), (T1, 1, 4, 3), (T2, 2, 6, 5));
#[rustfmt::skip]
lower_tuple!((T0, 0, 2, 1), (T1, 1, 4, 3), (T2, 2, 6, 5), (T3, 3, 8, 7));
#[rustfmt::skip]
lower_tuple!((T0, 0, 2, 1), (T1, 1, 4, 3), (T2, 2, 6, 5), (T3, 3, 8, 7), (T4, 4, 10, 9));
#[rustfmt::skip]
lower_tuple!((T0, 0, 2, 1), (T1, 1, 4, 3), (T2, 2, 6, 5), (T3, 3, 8, 7), (T4, 4, 10, 9), (T5, 5, 12, 11));
#[rustfmt::skip]
lower_tuple!((T0, 0, 2, 1), (T1, 1, 4, 3), (T2, 2, 6, 5), (T3, 3, 8, 7), (T4, 4, 10, 9), (T5, 5, 12, 11), (T6, 6, 14, 13));
#[rustfmt::skip]
lower_tuple!((T0, 0, 2, 1), (T1, 1, 4, 3), (T2, 2, 6, 5), (T3, 3, 8, 7), (T4, 4, 10, 9), (T5, 5, 12, 11), (T6, 6, 14, 13), (T7, 7, 16, 15));
