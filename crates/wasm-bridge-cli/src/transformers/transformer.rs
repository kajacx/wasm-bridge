use crate::zipper::ZippableFile;

use anyhow::{Ok, Result};

pub type Transformer = Box<dyn Fn(&mut Vec<ZippableFile>) -> Result<()>>;

pub fn transform_all(files: &mut Vec<ZippableFile>, transformers: &[Transformer]) -> Result<()> {
    for transformer in transformers {
        transformer(files)?;
    }
    Ok(())
}
