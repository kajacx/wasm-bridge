use wasmtime::{component::Component, Engine, Error, Result};
use zip::ZipArchive;

pub fn new_universal_component(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
    Component::new(engine, bytes.as_ref()).or_else(|original_error| {
        try_load_universal_component(engine, bytes.as_ref())
            .map_err(|new_error| new_error.unwrap_or(original_error))
    })
}

fn try_load_universal_component(engine: &Engine, bytes: &[u8]) -> Result<Component, Option<Error>> {
    let cursor = std::io::Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).void_error()?;

    let mut file_bytes = vec![];

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let filename = file.name();

        if filename.ends_with("original_component.wasm") {
            std::io::copy(&mut file, &mut file_bytes).void_error()?;
        }
    }

    if file_bytes.is_empty() {
        return Err(None);
    }

    Component::new(engine, &file_bytes).map_err(Some)
}

trait VoidError<T> {
    fn void_error(self) -> Result<T, Option<Error>>;
}

impl<R, E> VoidError<R> for Result<R, E> {
    fn void_error(self) -> Result<R, Option<Error>> {
        self.map_err(|_| None)
    }
}
