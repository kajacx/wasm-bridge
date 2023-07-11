use std::{path::PathBuf, process::Command};

use anyhow::{anyhow, Context, Result};

use tempdir::TempDir;

pub fn run_jco_transpile(component: PathBuf) -> Result<TempDir> {
    if !component.is_file() {
        return Err(anyhow!(
            "SOURCE needs to be the .wasm component file in universal mode."
        ));
    }

    let dir = TempDir::new("jco_transpile")?;

    let output = Command::new("jco")
        .arg("transpile")
        .arg(
            component
                .as_os_str()
                .to_str()
                .context("Valid utf-8 component filename")?,
        )
        .arg("--instantiation")
        .arg("-0")
        .arg(
            dir.path()
                .as_os_str()
                .to_str()
                .context("Valid utf-8 temp dir filename")?,
        )
        .output()?;

    if !output.status.success() {
        Err(anyhow!(
            "Failed to run jco: {:?}. {} {}",
            output,
            "Make sure jco is installed.",
            "You can install jco with `npm install -g @bytecodealliance/jco`"
        ))
    } else {
        Ok(dir)
    }
}
