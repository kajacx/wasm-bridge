use crate::{transformers::Transformer, zipper::ZippableFile};

pub fn version_transformer() -> Transformer {
    Box::new(|files| {
        files.push(version_file());
        Ok(())
    })
}

fn version_file() -> ZippableFile {
    ZippableFile {
        name: "version.txt".into(),
        content: env!("CARGO_PKG_VERSION").as_bytes().to_owned(),
    }
}
