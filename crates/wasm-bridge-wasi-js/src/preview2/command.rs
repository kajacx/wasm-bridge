use crate::preview2::{clocks, WasiView};
use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use super::{cli, random, stream};

pub fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    stream::add_to_linker(linker)?;
    random::add_to_linker(linker)?;
    clocks::add_to_linker(linker)?;
    cli::add_to_linker(linker)?;

    Ok(())
}

wasm_bridge::component::bindgen!({
    // path: "wit/deps/cli/reactor.wit",
    world: "wasi:cli/command",
    // tracing: true,
    async: {
        only_imports: [
            "[method]descriptor.access-at",
            "[method]descriptor.advise",
            "[method]descriptor.change-directory-permissions-at",
            "[method]descriptor.change-file-permissions-at",
            "[method]descriptor.create-directory-at",
            "[method]descriptor.get-flags",
            "[method]descriptor.get-type",
            "[method]descriptor.is-same-object",
            "[method]descriptor.link-at",
            "[method]descriptor.lock-exclusive",
            "[method]descriptor.lock-shared",
            "[method]descriptor.metadata-hash",
            "[method]descriptor.metadata-hash-at",
            "[method]descriptor.open-at",
            "[method]descriptor.read",
            "[method]descriptor.read-directory",
            "[method]descriptor.readlink-at",
            "[method]descriptor.remove-directory-at",
            "[method]descriptor.rename-at",
            "[method]descriptor.set-size",
            "[method]descriptor.set-times",
            "[method]descriptor.set-times-at",
            "[method]descriptor.stat",
            "[method]descriptor.stat-at",
            "[method]descriptor.symlink-at",
            "[method]descriptor.sync",
            "[method]descriptor.sync-data",
            "[method]descriptor.try-lock-exclusive",
            "[method]descriptor.try-lock-shared",
            "[method]descriptor.unlink-file-at",
            "[method]descriptor.unlock",
            "[method]descriptor.write",
            "[method]input-stream.read",
            "[method]input-stream.blocking-read",
            "[method]input-stream.blocking-skip",
            "[method]input-stream.skip",
            "[method]output-stream.forward",
            "[method]output-stream.splice",
            "[method]output-stream.blocking-splice",
            "[method]output-stream.blocking-flush",
            "[method]output-stream.blocking-write",
            "[method]output-stream.blocking-write-and-flush",
            "[method]output-stream.blocking-write-zeroes-and-flush",
            "[method]directory-entry-stream.read-directory-entry",
            "poll",
            "[method]pollable.block",
            "[method]pollable.ready",
        ],
    },
});
