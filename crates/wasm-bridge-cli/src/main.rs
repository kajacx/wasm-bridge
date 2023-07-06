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
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut data = Vec::<u8>::new();
    let cursor = Cursor::new(&mut data);

    let mut writer = ZipWriter::new(cursor);

    let source_dir = std::fs::read_dir(args.source_dir)?;

    for file in source_dir {
        let file = file?;

        writer.start_file(
            file.file_name().to_str().expect("utf-8 file name"),
            FileOptions::default(),
        )?;

        let file_bytes = std::fs::read(file.path())?;
        writer.write_all(&file_bytes)?;
    }

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
