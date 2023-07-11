use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::zipper::ZippableFile;

pub fn read_files_from_dir(dir: &Path) -> Result<Vec<ZippableFile>> {
    let source_dir = std::fs::read_dir(dir)?;
    let mut results = vec![];

    for file in source_dir {
        results.push(read_path_buf(&file?.path())?);
    }

    Ok(results)
}

pub fn read_path_buf(path: &PathBuf) -> Result<ZippableFile> {
    let file_name = path.file_name().context("get file name")?;
    let file_name = file_name.to_str().context("file name is valid utf-8")?;

    let file_bytes = std::fs::read(path)?;

    Ok(ZippableFile {
        name: file_name.into(),
        content: file_bytes,
    })
}
