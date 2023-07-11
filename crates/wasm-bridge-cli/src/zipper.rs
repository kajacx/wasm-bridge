use std::io::{Cursor, Write};

use anyhow::Result;
use zip::{write::FileOptions, ZipWriter};

#[derive(Clone, Debug)]
pub struct ZippableFile {
    pub name: String,
    pub content: Vec<u8>,
}

pub fn create_zip(files: &[ZippableFile]) -> Result<Vec<u8>> {
    let mut data = Vec::<u8>::new();
    let cursor = Cursor::new(&mut data);
    let mut writer = ZipWriter::new(cursor);

    for file in files {
        writer.start_file(&file.name, FileOptions::default())?;
        writer.write_all(&file.content)?;
    }

    writer.finish()?;
    drop(writer);

    Ok(data)
}
