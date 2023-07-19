use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::zipper::ZippableFile;

pub fn read_files_from_dir(dir: &Path) -> Result<Vec<ZippableFile>> {
    let source_dir = std::fs::read_dir(dir)?;
    let mut results = vec![];

    for file in source_dir {
        results.extend(read_path_buf(&file?.path())?);
    }

    Ok(results)
}

pub fn read_path_buf(path: &PathBuf) -> Result<Vec<ZippableFile>> {
    let file_name = path.file_name().context("get file name")?;
    let file_name = file_name.to_str().context("file name is valid utf-8")?;

    if path.is_file() {
        let file_bytes = std::fs::read(path)?;
        Ok(vec![ZippableFile {
            name: file_name.into(),
            content: file_bytes,
        }])
    } else if path.is_dir() {
        // TODO: this "flattens" the directory structure, which is not great
        read_files_from_dir(path)
    } else {
        anyhow::bail!("Path is not a file nor a dir: {path:?}")
    }
}
