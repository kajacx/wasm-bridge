use std::{fs, path::Path};

use wasmtime::{component::__internal::anyhow::Context, Engine, Result};

#[derive(Clone)]
pub struct Component(wasmtime::component::Component);

impl Component {
    /// Compiles a new WebAssembly component from the in-memory wasm image
    /// provided.
    pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
        // TODO
        todo!()
    }

    /// Compiles a new WebAssembly component from a wasm file on disk pointed to
    /// by `file`.
    pub fn from_file(engine: &Engine, file: impl AsRef<Path>) -> Result<Component> {
        match Self::new(
            engine,
            &fs::read(&file).with_context(|| "failed to read input file")?,
        ) {
            Ok(m) => Ok(m),
            Err(e) => {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "wat")] {
                        let mut e = e.downcast::<wat::Error>()?;
                        e.set_path(file);
                        bail!(e)
                    } else {
                        Err(e)
                    }
                }
            }
        }
    }

    /// Compiles a new WebAssembly component from the in-memory wasm image
    /// provided.
    pub fn from_binary(engine: &Engine, binary: &[u8]) -> Result<Component> {
        engine
            .check_compatible_with_native_host()
            .context("compilation settings are not compatible with the native host")?;

        let (mmap, artifacts) = Component::build_artifacts(engine, binary)?;
        let mut code_memory = CodeMemory::new(mmap)?;
        code_memory.publish()?;
        Component::from_parts(engine, Arc::new(code_memory), Some(artifacts))
    }

    /// Same as [`Module::deserialize`], but for components.
    ///
    /// Note that the file referenced here must contain contents previously
    /// produced by [`Engine::precompile_component`] or
    /// [`Component::serialize`].
    ///
    /// For more information see the [`Module::deserialize`] method.
    ///
    /// [`Module::deserialize`]: crate::Module::deserialize
    pub unsafe fn deserialize(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
        wasmtime::component::Component::deserialize(engine, bytes)
    }

    /// Same as [`Module::deserialize_file`], but for components.
    ///
    /// For more information see the [`Component::deserialize`] and
    /// [`Module::deserialize_file`] methods.
    ///
    /// [`Module::deserialize_file`]: crate::Module::deserialize_file
    pub unsafe fn deserialize_file(engine: &Engine, path: impl AsRef<Path>) -> Result<Component> {
        wasmtime::component::Component::deserialize(engine, path)
    }

    /// Same as [`Module::serialize`], except for a component.
    ///
    /// Note that the artifact produced here must be passed to
    /// [`Component::deserialize`] and is not compatible for use with
    /// [`Module`].
    ///
    /// [`Module::serialize`]: crate::Module::serialize
    /// [`Module`]: crate::Module
    pub fn serialize(&self) -> Result<Vec<u8>> {
        self.0.serialize()
    }
}
