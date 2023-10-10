pub mod preview2;

wasm_bridge::component::bindgen! ({
    path: "./wit",
    world: "preview1-adapter-reactor",
    async: false,
});
