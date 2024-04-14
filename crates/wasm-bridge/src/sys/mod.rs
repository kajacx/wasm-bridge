use ref_cast::RefCast;

pub use wasmtime::*;

// DISCLAIMER: All annotations are directly copied from wasmtime.

/// A compiled WebAssembly module, ready to be instantiated.
///
/// A `Module` is a compiled in-memory representation of an input WebAssembly
/// binary. A `Module` is then used to create an [`Instance`](crate::Instance)
/// through an instantiation process. You cannot call functions or fetch
/// globals, for example, on a `Module` because it's purely a code
/// representation. Instead you'll need to create an
/// [`Instance`](crate::Instance) to interact with the wasm module.
///
/// A `Module` can be created by compiling WebAssembly code through APIs such as
/// [`Module::new`]. This would be a JIT-style use case where code is compiled
/// just before it's used. Alternatively a `Module` can be compiled in one
/// process and [`Module::serialize`] can be used to save it to storage. A later
/// call to [`Module::deserialize`] will quickly load the module to execute and
/// does not need to compile any code, representing a more AOT-style use case.
///
/// Currently a `Module` does not implement any form of tiering or dynamic
/// optimization of compiled code. Creation of a `Module` via [`Module::new`] or
/// related APIs will perform the entire compilation step synchronously. When
/// finished no further compilation will happen at runtime or later during
/// execution of WebAssembly instances for example.
///
/// Compilation of WebAssembly by default goes through Cranelift and is
/// recommended to be done once-per-module. The same WebAssembly binary need not
/// be compiled multiple times and can instead used an embedder-cached result of
/// the first call.
///
/// `Module` is thread-safe and safe to share across threads.
///
/// ## Modules and `Clone`
///
/// Using `clone` on a `Module` is a cheap operation. It will not create an
/// entirely new module, but rather just a new reference to the existing module.
/// In other words it's a shallow copy, not a deep copy.
///
/// ## Examples
///
/// There are a number of ways you can create a `Module`, for example pulling
/// the bytes from a number of locations. One example is loading a module from
/// the filesystem:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// let module = Module::from_file(&engine, "path/to/foo.wasm")?;
/// # Ok(())
/// # }
/// ```
///
/// You can also load the wasm text format if more convenient too:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// // Now we're using the WebAssembly text extension: `.wat`!
/// let module = Module::from_file(&engine, "path/to/foo.wat")?;
/// # Ok(())
/// # }
/// ```
///
/// And if you've already got the bytes in-memory you can use the
/// [`Module::new`] constructor:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// # let wasm_bytes: Vec<u8> = Vec::new();
/// let module = Module::new(&engine, &wasm_bytes)?;
///
/// // It also works with the text format!
/// let module = Module::new(&engine, "(module (func))")?;
/// # Ok(())
/// # }
/// ```
///
/// Serializing and deserializing a module looks like:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// # let wasm_bytes: Vec<u8> = Vec::new();
/// let module = Module::new(&engine, &wasm_bytes)?;
/// let module_bytes = module.serialize()?;
///
/// // ... can save `module_bytes` to disk or other storage ...
///
/// // recreate the module from the serialized bytes. For the `unsafe` bits
/// // see the documentation of `deserialize`.
/// let module = unsafe { Module::deserialize(&engine, &module_bytes)? };
/// # Ok(())
/// # }
/// ```
///
/// [`Config`]: crate::Config
#[repr(transparent)]
#[derive(RefCast, Clone)]
pub struct Module(pub(crate) wasmtime::Module);

impl Module {
    #[deprecated(
        since = "0.4.0",
        note = "Compiling a module synchronously can panic on the web, please use `new_safe` instead."
    )]
    pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        Ok(Self(wasmtime::Module::new(engine, bytes)?))
    }

    /// Creates a new WebAssembly `Module` from the given in-memory `bytes`.
    ///
    /// The `bytes` provided must be in one of the following formats:
    ///
    /// * A [binary-encoded][binary] WebAssembly module. This is always supported.
    /// * A [text-encoded][text] instance of the WebAssembly text format.
    ///   This is only supported when the `wat` feature of this crate is enabled.
    ///   If this is supplied then the text format will be parsed before validation.
    ///   Note that the `wat` feature is enabled by default.
    ///
    /// The data for the wasm module must be loaded in-memory if it's present
    /// elsewhere, for example on disk. This requires that the entire binary is
    /// loaded into memory all at once, this API does not support streaming
    /// compilation of a module.
    ///
    /// The WebAssembly binary will be decoded and validated. It will also be
    /// compiled according to the configuration of the provided `engine`.
    ///
    /// # Errors
    ///
    /// This function may fail and return an error. Errors may include
    /// situations such as:
    ///
    /// * The binary provided could not be decoded because it's not a valid
    ///   WebAssembly binary
    /// * The WebAssembly binary may not validate (e.g. contains type errors)
    /// * Implementation-specific limits were exceeded with a valid binary (for
    ///   example too many locals)
    /// * The wasm binary may use features that are not enabled in the
    ///   configuration of `engine`
    /// * If the `wat` feature is enabled and the input is text, then it may be
    ///   rejected if it fails to parse.
    ///
    /// The error returned should contain full information about why module
    /// creation failed if one is returned.
    ///
    /// [binary]: https://webassembly.github.io/spec/core/binary/index.html
    /// [text]: https://webassembly.github.io/spec/core/text/index.html
    ///
    /// # Examples
    ///
    /// The `new` function can be invoked with a in-memory array of bytes:
    ///
    /// ```no_run
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let wasm_bytes: Vec<u8> = Vec::new();
    /// let module = Module::new(&engine, &wasm_bytes)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Or you can also pass in a string to be parsed as the wasm text
    /// format:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let module = Module::new(&engine, "(module (func))")?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_safe(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        // This just calls `new` on sys, but uses proper async compilation on the web.
        #[allow(deprecated)]
        Self::new(engine, bytes)
    }
}

/// An instantiated WebAssembly module.
///
/// This type represents the instantiation of a [`Module`]. Once instantiated
/// you can access the [`exports`](Instance::exports) which are of type
/// [`Extern`] and provide the ability to call functions, set globals, read
/// memory, etc. When interacting with any wasm code you'll want to make an
/// [`Instance`] to call any code or execute anything.
///
/// Instances are owned by a [`Store`](crate::Store) which is passed in at
/// creation time. It's recommended to create instances with
/// [`Linker::instantiate`](crate::Linker::instantiate) or similar
/// [`Linker`](crate::Linker) methods, but a more low-level constructor is also
/// available as [`Instance::new`].
#[repr(transparent)]
#[derive(RefCast, Clone, Debug)]
pub struct Instance(pub(crate) wasmtime::Instance);

impl Instance {
    #[deprecated(
        since = "0.4.0",
        note = "Instantiating a module synchronously can panic on the web, please use `new_safe` instead."
    )]
    pub fn new(store: impl AsContextMut, module: &Module, imports: &[Extern]) -> Result<Self> {
        Ok(Self(wasmtime::Instance::new(store, &module.0, imports)?))
    }

    /// Creates a new [`Instance`] from the previously compiled [`Module`] and
    /// list of `imports` specified.
    ///
    /// This method instantiates the `module` provided with the `imports`,
    /// following the procedure in the [core specification][inst] to
    /// instantiate. Instantiation can fail for a number of reasons (many
    /// specified below), but if successful the `start` function will be
    /// automatically run (if specified in the `module`) and then the
    /// [`Instance`] will be returned.
    ///
    /// Per the WebAssembly spec, instantiation includes running the module's
    /// start function, if it has one (not to be confused with the `_start`
    /// function, which is not run).
    ///
    /// Note that this is a low-level function that just performs an
    /// instantiation. See the [`Linker`](crate::Linker) struct for an API which
    /// provides a convenient way to link imports and provides automatic Command
    /// and Reactor behavior.
    ///
    /// ## Providing Imports
    ///
    /// The entries in the list of `imports` are intended to correspond 1:1
    /// with the list of imports returned by [`Module::imports`]. Before
    /// calling [`Instance::new`] you'll want to inspect the return value of
    /// [`Module::imports`] and, for each import type, create an [`Extern`]
    /// which corresponds to that type.  These [`Extern`] values are all then
    /// collected into a list and passed to this function.
    ///
    /// Note that this function is intentionally relatively low level. For an
    /// easier time passing imports by doing name-based resolution it's
    /// recommended to instead use the [`Linker`](crate::Linker) type.
    ///
    /// ## Errors
    ///
    /// This function can fail for a number of reasons, including, but not
    /// limited to:
    ///
    /// * The number of `imports` provided doesn't match the number of imports
    ///   returned by the `module`'s [`Module::imports`] method.
    /// * The type of any [`Extern`] doesn't match the corresponding
    ///   [`ExternType`] entry that it maps to.
    /// * The `start` function in the instance, if present, traps.
    /// * Module/instance resource limits are exceeded.
    ///
    /// When instantiation fails it's recommended to inspect the return value to
    /// see why it failed, or bubble it upwards. If you'd like to specifically
    /// check for trap errors, you can use `error.downcast::<Trap>()`. For more
    /// about error handling see the [`Trap`] documentation.
    ///
    /// [`Trap`]: crate::Trap
    ///
    /// # Panics
    ///
    /// This function will panic if called with a store associated with a
    /// [`asynchronous config`](crate::Config::async_support). This function
    /// will also panic if any [`Extern`] supplied is not owned by `store`.
    ///
    /// [inst]: https://webassembly.github.io/spec/core/exec/modules.html#exec-instantiation
    /// [`ExternType`]: crate::ExternType
    pub async fn new_safe(
        store: impl AsContextMut,
        module: &Module,
        imports: &[Extern],
    ) -> Result<Self> {
        // This just calls `new` on sys, but uses proper async instantiation on the web.
        #[allow(deprecated)]
        Self::new(store, module, imports)
    }

    /// Same as [`Instance::new`], except for usage in [asynchronous stores].
    ///
    /// For more details about this function see the documentation on
    /// [`Instance::new`]. The only difference between these two methods is that
    /// this one will asynchronously invoke the wasm start function in case it
    /// calls any imported function which is an asynchronous host function (e.g.
    /// created with [`Func::new_async`](crate::Func::new_async).
    ///
    /// # Panics
    ///
    /// This function will panic if called with a store associated with a
    /// [`synchronous config`](crate::Config::new). This is only compatible with
    /// stores associated with an [`asynchronous
    /// config`](crate::Config::async_support).
    ///
    /// This function will also panic, like [`Instance::new`], if any [`Extern`]
    /// specified does not belong to `store`.
    #[cfg(feature = "async")]
    pub async fn new_async<T>(
        store: impl AsContextMut<Data = T>,
        module: &Module,
        imports: &[Extern],
    ) -> Result<Self>
    where
        T: Send,
    {
        Ok(Self(
            wasmtime::Instance::new_async(store, &module.0, imports).await?,
        ))
    }

    /// Looks up an exported [`Memory`] value by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a memory.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_memory(&self, store: impl AsContextMut, name: &str) -> Option<Memory> {
        self.0.get_memory(store, name)
    }

    /// Looks up an exported [`Func`] value by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a function.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_func(&self, store: impl AsContextMut, name: &str) -> Option<Func> {
        self.0.get_func(store, name)
    }

    /// Looks up an exported [`Func`] value by name and with its type.
    ///
    /// This function is a convenience wrapper over [`Instance::get_func`] and
    /// [`Func::typed`]. For more information see the linked documentation.
    ///
    /// Returns an error if `name` isn't a function export or if the export's
    /// type did not match `Params` or `Results`
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_typed_func<Params, Results>(
        &self,
        store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>>
    where
        Params: WasmParams,
        Results: WasmResults,
    {
        self.0.get_typed_func(store, name)
    }
}

/// Structure used to link wasm modules/instances together.
///
/// This structure is used to assist in instantiating a [`Module`]. A [`Linker`]
/// is a way of performing name resolution to make instantiating a module easier
/// than specifying positional imports to [`Instance::new`]. [`Linker`] is a
/// name-based resolver where names are dynamically defined and then used to
/// instantiate a [`Module`].
///
/// An important method is [`Linker::instantiate`] which takes a module to
/// instantiate into the provided store. This method will automatically select
/// all the right imports for the [`Module`] to be instantiated, and will
/// otherwise return an error if an import isn't satisfied.
///
/// ## Name Resolution
///
/// As mentioned previously, `Linker` is a form of name resolver. It will be
/// using the string-based names of imports on a module to attempt to select a
/// matching item to hook up to it. This name resolution has two-levels of
/// namespaces, a module level and a name level. Each item is defined within a
/// module and then has its own name. This basically follows the wasm standard
/// for modularization.
///
/// Names in a `Linker` cannot be defined twice, but allowing duplicates by
/// shadowing the previous definition can be controlled with the
/// [`Linker::allow_shadowing`] method.
///
/// ## Commands and Reactors
///
/// The [`Linker`] type provides conveniences for working with WASI Commands and
/// Reactors through the [`Linker::module`] method. This will automatically
/// handle instantiation and calling `_start` and such as appropriate
/// depending on the inferred type of module.
///
/// ## Type parameter `T`
///
/// It's worth pointing out that the type parameter `T` on [`Linker<T>`] does
/// not represent that `T` is stored within a [`Linker`]. Rather the `T` is used
/// to ensure that linker-defined functions and stores instantiated into all use
/// the same matching `T` as host state.
///
/// ## Multiple `Store`s
///
/// The [`Linker`] type is designed to be compatible, in some scenarios, with
/// instantiation in multiple [`Store`]s. Specifically host-defined functions
/// created in [`Linker`] with [`Linker::func_new`], [`Linker::func_wrap`], and
/// their async versions are compatible to instantiate into any [`Store`]. This
/// enables programs which want to instantiate lots of modules to create one
/// [`Linker`] value at program start up and use that continuously for each
/// [`Store`] created over the lifetime of the program.
///
/// Note that once [`Store`]-owned items, such as [`Global`], are defined witin
/// a [`Linker`] then it is no longer compatible with any [`Store`]. At that
/// point only the [`Store`] that owns the [`Global`] can be used to instantiate
/// modules.
///
/// ## Multiple `Engine`s
///
/// The [`Linker`] type is not compatible with usage between multiple [`Engine`]
/// values. An [`Engine`] is provided when a [`Linker`] is created and only
/// stores and items which originate from that [`Engine`] can be used with this
/// [`Linker`]. If more than one [`Engine`] is used with a [`Linker`] then that
/// may cause a panic at runtime, similar to how if a [`Func`] is used with the
/// wrong [`Store`] that can also panic at runtime.
///
/// [`Store`]: crate::Store
/// [`Global`]: crate::Global
#[repr(transparent)]
#[derive(RefCast, Clone)]
pub struct Linker<T>(pub(crate) wasmtime::Linker<T>);

impl<T> Linker<T> {
    /// Creates a new [`Linker`].
    ///
    /// The linker will define functions within the context of the `engine`
    /// provided and can only instantiate modules for a [`Store`][crate::Store]
    /// that is also defined within the same [`Engine`]. Usage of stores with
    /// different [`Engine`]s may cause a panic when used with this [`Linker`].
    pub fn new(engine: &Engine) -> Self {
        Self(wasmtime::Linker::new(engine))
    }

    #[deprecated(
        since = "0.4.0",
        note = "Instantiating a module synchronously can panic on the web, please use `instantiate_safe` instead."
    )]
    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance, Error> {
        Ok(Instance(self.0.instantiate(store, &module.0)?))
    }

    /// Attempts to instantiate the `module` provided.
    ///
    /// This method will attempt to assemble a list of imports that correspond
    /// to the imports required by the [`Module`] provided. This list
    /// of imports is then passed to [`Instance::new`] to continue the
    /// instantiation process.
    ///
    /// Each import of `module` will be looked up in this [`Linker`] and must
    /// have previously been defined. If it was previously defined with an
    /// incorrect signature or if it was not previously defined then an error
    /// will be returned because the import can not be satisfied.
    ///
    /// Per the WebAssembly spec, instantiation includes running the module's
    /// start function, if it has one (not to be confused with the `_start`
    /// function, which is not run).
    ///
    /// # Errors
    ///
    /// This method can fail because an import may not be found, or because
    /// instantiation itself may fail. For information on instantiation
    /// failures see [`Instance::new`]. If an import is not found, the error
    /// may be downcast to an [`UnknownImportError`].
    ///
    ///
    /// # Panics
    ///
    /// Panics if any item used to instantiate `module` is not owned by
    /// `store`. Additionally this will panic if the [`Engine`] that the `store`
    /// belongs to is different than this [`Linker`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// linker.func_wrap("host", "double", |x: i32| x * 2)?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "host" "double" (func (param i32) (result i32)))
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// linker.instantiate(&mut store, &module)?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn instantiate_safe(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance, Error> {
        // This just calls `instantiate` on sys, but uses proper async instantiation on the web.
        #[allow(deprecated)]
        self.instantiate(store, module)
    }

    /// Attempts to instantiate the `module` provided. This is the same as
    /// [`Linker::instantiate`], except for async `Store`s.
    #[cfg(feature = "async")]
    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance>
    where
        T: Send,
    {
        Ok(Instance(self.0.instantiate_async(store, &module.0).await?))
    }

    /// Creates a [`Func::new`]-style function named in this linker.
    ///
    /// For more information see [`Linker::func_wrap`].
    ///
    /// # Panics
    ///
    /// Panics if the given function type is not associated with the same engine
    /// as this linker.
    pub fn func_new(
        &mut self,
        module: &str,
        name: &str,
        ty: FuncType,
        func: impl Fn(Caller<T>, &[Val], &mut [Val]) -> Result<()> + Send + Sync + 'static,
    ) -> Result<&mut Self> {
        Ok(Self::ref_cast_mut(self.0.func_new(module, name, ty, func)?))
    }

    /// Define a host function within this linker.
    ///
    /// For information about how the host function operates, see
    /// [`Func::wrap`]. That includes information about translating Rust types
    /// to WebAssembly native types.
    ///
    /// This method creates a host-provided function in this linker under the
    /// provided name. This method is distinct in its capability to create a
    /// [`Store`](crate::Store)-independent function. This means that the
    /// function defined here can be used to instantiate instances in multiple
    /// different stores, or in other words the function can be loaded into
    /// different stores.
    ///
    /// Note that the capability mentioned here applies to all other
    /// host-function-defining-methods on [`Linker`] as well. All of them can be
    /// used to create instances of [`Func`] within multiple stores. In a
    /// multithreaded program, for example, this means that the host functions
    /// could be called concurrently if different stores are executing on
    /// different threads.
    ///
    /// # Errors
    ///
    /// Returns an error if the `module` and `name` already identify an item
    /// of the same type as the `item` provided and if shadowing is disallowed.
    /// For more information see the documentation on [`Linker`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let mut linker = Linker::new(&engine);
    /// linker.func_wrap("host", "double", |x: i32| x * 2)?;
    /// linker.func_wrap("host", "log_i32", |x: i32| println!("{}", x))?;
    /// linker.func_wrap("host", "log_str", |caller: Caller<'_, ()>, ptr: i32, len: i32| {
    ///     // ...
    /// })?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "host" "double" (func (param i32) (result i32)))
    ///         (import "host" "log_i32" (func (param i32)))
    ///         (import "host" "log_str" (func (param i32 i32)))
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    ///
    /// // instantiate in multiple different stores
    /// for _ in 0..10 {
    ///     let mut store = Store::new(&engine, ());
    ///     linker.instantiate(&mut store, &module)?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn func_wrap<Params, Results>(
        &mut self,
        module: &str,
        name: &str,
        func: impl IntoFunc<T, Params, Results>,
    ) -> Result<&mut Self> {
        Ok(Self::ref_cast_mut(self.0.func_wrap(module, name, func)?))
    }
}

#[cfg(feature = "component-model")]
pub mod component {
    pub use wasmtime::component::*;

    pub use wasm_bridge_macros::bindgen_sys as bindgen;
    pub use wasm_bridge_macros::flags_sys as flags;
    pub use wasm_bridge_macros::ComponentType;
    pub use wasm_bridge_macros::Lift;
    pub use wasm_bridge_macros::Lower;

    use ref_cast::RefCast;
    use wasmtime::{AsContextMut, Engine, Result};

    /// A compiled WebAssembly Component.
    ///
    /// This structure represents a compiled component that is ready to be
    /// instantiated. This owns a region of virtual memory which contains executable
    /// code compiled from a WebAssembly binary originally. This is the analog of
    /// [`Module`](crate::Module) in the component embedding API.
    ///
    /// A [`Component`] can be turned into an
    /// [`Instance`](crate::component::Instance) through a
    /// [`Linker`](crate::component::Linker). [`Component`]s are safe to share
    /// across threads. The compilation model of a component is the same as that of
    /// [a module](crate::Module) which is to say:
    ///
    /// * Compilation happens synchronously during [`Component::new`].
    /// * The result of compilation can be saved into storage with
    ///   [`Component::serialize`].
    /// * A previously compiled artifact can be parsed with
    ///   [`Component::deserialize`].
    /// * No compilation happens at runtime for a component — everything is done
    ///   by the time [`Component::new`] returns.
    ///
    /// ## Components and `Clone`
    ///
    /// Using `clone` on a `Component` is a cheap operation. It will not create an
    /// entirely new component, but rather just a new reference to the existing
    /// component. In other words it's a shallow copy, not a deep copy.
    ///
    /// ## Examples
    ///
    /// For example usage see the documentation of [`Module`](crate::Module) as
    /// [`Component`] has the same high-level API.
    #[repr(transparent)]
    #[derive(RefCast, Clone)]
    pub struct Component(pub(crate) wasmtime::component::Component);

    impl Component {
        #[deprecated(
            since = "0.4.0",
            note = "Compiling a component synchronously can panic on the web, please use `new_safe` instead."
        )]
        pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
            Ok(Self(wasmtime::component::Component::new(engine, bytes)?))
        }

        /// Compiles a new WebAssembly component from the in-memory list of bytes
        /// provided.
        ///
        /// The `bytes` provided can either be the binary or text format of a
        /// [WebAssembly component]. Note that the text format requires the `wat`
        /// feature of this crate to be enabled. This API does not support
        /// streaming compilation.
        ///
        /// This function will synchronously validate the entire component,
        /// including all core modules, and then compile all components, modules,
        /// etc., found within the provided bytes.
        ///
        /// [WebAssembly component]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md
        ///
        /// # Errors
        ///
        /// This function may fail and return an error. Errors may include
        /// situations such as:
        ///
        /// * The binary provided could not be decoded because it's not a valid
        ///   WebAssembly binary
        /// * The WebAssembly binary may not validate (e.g. contains type errors)
        /// * Implementation-specific limits were exceeded with a valid binary (for
        ///   example too many locals)
        /// * The wasm binary may use features that are not enabled in the
        ///   configuration of `engine`
        /// * If the `wat` feature is enabled and the input is text, then it may be
        ///   rejected if it fails to parse.
        ///
        /// The error returned should contain full information about why compilation
        /// failed.
        ///
        /// # Examples
        ///
        /// The `new` function can be invoked with a in-memory array of bytes:
        ///
        /// ```no_run
        /// # use wasmtime::*;
        /// # use wasmtime::component::Component;
        /// # fn main() -> anyhow::Result<()> {
        /// # let engine = Engine::default();
        /// # let wasm_bytes: Vec<u8> = Vec::new();
        /// let component = Component::new(&engine, &wasm_bytes)?;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// Or you can also pass in a string to be parsed as the wasm text
        /// format:
        ///
        /// ```
        /// # use wasmtime::*;
        /// # use wasmtime::component::Component;
        /// # fn main() -> anyhow::Result<()> {
        /// # let engine = Engine::default();
        /// let component = Component::new(&engine, "(component (core module))")?;
        /// # Ok(())
        /// # }
        pub async fn new_safe(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
            // This just calls `new` on sys, but uses proper async compilation on the web.
            #[allow(deprecated)]
            Self::new(engine, bytes)
        }
    }

    /// A type used to instantiate [`Component`]s.
    ///
    /// This type is used to both link components together as well as supply host
    /// functionality to components. Values are defined in a [`Linker`] by their
    /// import name and then components are instantiated with a [`Linker`] using the
    /// names provided for name resolution of the component's imports.
    ///
    /// # Names and Semver
    ///
    /// Names defined in a [`Linker`] correspond to import names in the Component
    /// Model. Names in the Component Model are allowed to be semver-qualified, for
    /// example:
    ///
    /// * `wasi:cli/stdout@0.2.0`
    /// * `wasi:http/types@0.2.0-rc-2023-10-25`
    /// * `my:custom/plugin@1.0.0-pre.2`
    ///
    /// These version strings are taken into account when looking up names within a
    /// [`Linker`]. You're allowed to define any number of versions within a
    /// [`Linker`] still, for example you can define `a:b/c@0.2.0`, `a:b/c@0.2.1`,
    /// and `a:b/c@0.3.0` all at the same time.
    ///
    /// Specifically though when names are looked up within a linker, for example
    /// during instantiation, semver-compatible names are automatically consulted.
    /// This means that if you define `a:b/c@0.2.1` in a [`Linker`] but a component
    /// imports `a:b/c@0.2.0` then that import will resolve to the `0.2.1` version.
    ///
    /// This lookup behavior relies on hosts being well-behaved when using Semver,
    /// specifically that interfaces once defined are never changed. This reflects
    /// how Semver works at the Component Model layer, and it's assumed that if
    /// versions are present then hosts are respecting this.
    ///
    /// Note that this behavior goes the other direction, too. If a component
    /// imports `a:b/c@0.2.1` and the host has provided `a:b/c@0.2.0` then that
    /// will also resolve correctly. This is because if an API was defined at 0.2.0
    /// and 0.2.1 then it must be the same API.
    ///
    /// This behavior is intended to make it easier for hosts to upgrade WASI and
    /// for guests to upgrade WASI. So long as the actual "meat" of the
    /// functionality is defined then it should align correctly and components can
    /// be instantiated.
    #[repr(transparent)]
    #[derive(RefCast, Clone)]
    pub struct Linker<T>(pub wasmtime::component::Linker<T>);

    impl<T> Linker<T> {
        /// Creates a new linker for the [`Engine`] specified with no items defined
        /// within it.
        pub fn new(engine: &Engine) -> Self {
            Self(wasmtime::component::Linker::new(engine))
        }

        #[deprecated(
            since = "0.4.0",
            note = "Instantiating a component synchronously can panic on the web, please use `instantiate_safe` instead."
        )]
        pub fn instantiate(
            &self,
            store: impl AsContextMut<Data = T>,
            component: &Component,
        ) -> Result<Instance> {
            self.0.instantiate(store, &component.0)
        }

        /// Instantiates the [`Component`] provided into the `store` specified.
        ///
        /// This function will use the items defined within this [`Linker`] to
        /// satisfy the imports of the [`Component`] provided as necessary. For more
        /// information about this see [`Linker::instantiate_pre`] as well.
        ///
        /// # Errors
        ///
        /// Returns an error if this [`Linker`] doesn't define an import that
        /// `component` requires or if it is of the wrong type. Additionally this
        /// can return an error if something goes wrong during instantiation such as
        /// a runtime trap or a runtime limit being exceeded.
        pub async fn instantiate_safe(
            &self,
            store: impl AsContextMut<Data = T>,
            component: &Component,
        ) -> Result<Instance> {
            // This just calls `instantiate` on sys, but uses proper async instantiation on the web.
            #[allow(deprecated)]
            self.instantiate(store, component)
        }

        /// Instantiates the [`Component`] provided into the `store` specified.
        ///
        /// This is exactly like [`Linker::instantiate`] except for async stores.
        ///
        /// # Errors
        ///
        /// Returns an error if this [`Linker`] doesn't define an import that
        /// `component` requires or if it is of the wrong type. Additionally this
        /// can return an error if something goes wrong during instantiation such as
        /// a runtime trap or a runtime limit being exceeded.:w
        ///
        #[cfg(feature = "async")]
        pub async fn instantiate_async(
            &self,
            store: impl AsContextMut<Data = T>,
            component: &Component,
        ) -> Result<Instance>
        where
            T: Send,
        {
            self.0.instantiate_async(store, &component.0).await
        }

        /// Returns the "root instance" of this linker, used to define names into
        /// the root namespace.
        pub fn root(&mut self) -> LinkerInstance<T> {
            self.0.root()
        }

        /// Returns a builder for the named instance specified.
        ///
        /// # Errors
        ///
        /// Returns an error if `name` is already defined within the linker.
        pub fn instance(&mut self, name: &str) -> Result<LinkerInstance<T>> {
            self.0.instance(name)
        }
    }
}

#[cfg(feature = "async")]
pub use async_trait::async_trait;
