use std::io::{Bytes, Cursor, Write};

use clap::Parser;
use zip::{write::FileOptions, ZipWriter};

#[derive(Parser)]
struct Args {
    source_dir: std::path::PathBuf,
    out_file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut data = Vec::<u8>::new();
    let cursor = Cursor::new(&mut data);

    let mut writer = ZipWriter::new(cursor);

    writer
        .start_file("hello.txt", FileOptions::default())
        .unwrap();
    writer.write_all("Hello world!".as_bytes()).unwrap();

    writer
        .start_file("world.txt", FileOptions::default())
        .unwrap();
    writer.write_all("Hello WORLD!".as_bytes()).unwrap();

    writer.finish().unwrap();
    drop(writer);

    std::fs::write(args.out_file.as_ref().unwrap(), data).unwrap();

    let bytes = std::fs::read(args.out_file.unwrap()).unwrap();

    let bytes = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(bytes).unwrap();

    println!("number of files: {}", archive.len());
}
