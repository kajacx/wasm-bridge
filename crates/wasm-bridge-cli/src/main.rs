use std::{
    error::Error,
    io::{Cursor, Write},
};

use clap::Parser;
use zip::{write::FileOptions, ZipWriter};

#[derive(Parser)]
struct Args {
    source_dir: std::path::PathBuf,
    out_file: Option<std::path::PathBuf>,
    // TODO: Add version
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut data = Vec::<u8>::new();
    let cursor = Cursor::new(&mut data);

    let mut writer = ZipWriter::new(cursor);

    let source_dir = std::fs::read_dir(args.source_dir)?;

    for file in source_dir {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_str().expect("utf-8 file name");

        let mut file_bytes = std::fs::read(file.path())?;
        if file_name == "component.js" {
            writer.start_file("sync_component.js", FileOptions::default())?;
            file_bytes = transform_component_js(file_bytes);
        } else {
            writer.start_file(file_name, FileOptions::default())?;
        }

        writer.write_all(&file_bytes)?;
    }

    writer.start_file("version.txt", FileOptions::default())?;
    writer.write_fmt(format_args!("{}", get_version()))?;

    writer.finish()?;
    drop(writer);

    match args.out_file {
        Some(out_file) => {
            std::fs::write(out_file, data)?;
        }
        None => {
            std::io::stdout().write_all(&data)?;
        }
    }

    Ok(())
}

fn transform_component_js(file_bytes: Vec<u8>) -> Vec<u8> {
    let text = String::from_utf8(file_bytes).expect("valid utf-8 component.js file");

    let text = text.replace("export async function", "function");
    let text = text.replace(
        "instantiateCore = WebAssembly.instantiate",
        "instantiateCore",
    );
    let text = text.replace("await ", "");

    let text = format!("(() => {{\n{text}\nreturn instantiate;\n}})()\n");
    text.into_bytes()
}

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}
