use std::fmt::Write;

use js_sys::Function;
use wasm_bindgen::JsValue;
use zip::ZipArchive;

use crate::{Engine, Result};

pub struct Component {
    pub(crate) _component: JsValue,
    pub(crate) _instantiate: Function,
}

impl Component {
    pub fn new(_engine: &Engine, bytes: &[u8]) -> Result<Self> {
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).unwrap();

        let mut text = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let filename = file.name();
            writeln!(text, "File name: {}", file.name()).unwrap();

            if filename.ends_with(".wasm") {
                let mut file_bytes = Vec::<u8>::new();

                std::io::copy(&mut file, &mut file_bytes).unwrap();

                writeln!(text, "File bytes: {}", file_bytes.len()).unwrap();
            }
        }

        todo!("{}", text)
    }
}
