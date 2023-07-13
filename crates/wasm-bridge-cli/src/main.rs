use std::{
    io::Write,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use jco_runner::run_jco_transpile;
use reader::read_files_from_dir;
use transformers::{add_original_file, js_transformer, transform_all, version_transformer};
use zipper::create_zip;

mod jco_runner;
mod reader;
mod transformers;
mod zipper;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Directory produced by jco
    source: PathBuf,

    /// Deprecated, use the -o option instead
    #[deprecated = "Will be removed in 0.2.0 release. Use -o [out_file] instead."]
    _out_file: Option<PathBuf>,

    /// Save output to this file instead of printing it to stdout.
    #[arg(short, long)]
    out_file: Option<PathBuf>,

    /// Build a universal component to be used with `wasm_bridge::component::from_universal`.
    /// Pass the WASM component file that you passed to jco.
    #[arg(short, long)]
    universal: Option<PathBuf>,

    /// Keep a copy of the original "component.js" file in the resulting zip.
    #[arg(short, long)]
    keep_original_js: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let out_dir = resolve_out_dir(&args);

    // TODO: how to run the jco command from Rust directly?
    // let directory = get_directory(args.source.clone(), args.universal.is_some())?;
    let directory = args.source;
    let mut contents = read_files_from_dir(directory.deref())?;

    let mut transformers = vec![js_transformer(args.keep_original_js), version_transformer()];
    if let Some(original) = args.universal {
        transformers.push(add_original_file(original));
    }

    transform_all(&mut contents, &transformers)?;

    let zip_content = create_zip(&contents)?;

    write_output(out_dir, &zip_content)?;

    Ok(())
}

fn resolve_out_dir(args: &Args) -> Option<PathBuf> {
    #[allow(deprecated)]
    match &args._out_file {
        Some(old) => match &args.out_file {
            Some(new) => {
                eprint!("You specified the out file both as the second argument ");
                eprintln!("and with the -o option.");
                eprintln!("The -o option has priority and will be used.");
                eprint!("Specifying out file as the second argument is deprecated ");
                eprintln!("and will be removed in version 0.2.0.");
                Some(new.clone())
            }
            None => {
                eprint!("Specifying the out file as the second argument is deprecated ");
                eprintln!("and will be removed in version 0.2.0.");
                eprintln!("Use the -o option instead.");
                Some(old.clone())
            }
        },
        None => args.out_file.as_ref().map(PathBuf::clone),
    }
}

#[allow(unused)]
fn get_directory(source: PathBuf, universal: bool) -> Result<Box<dyn AsRef<Path>>> {
    Ok(if universal {
        Box::new(run_jco_transpile(source)?)
    } else {
        Box::new(source)
    })
}

fn write_output(out_file: Option<PathBuf>, data: &[u8]) -> Result<()> {
    match out_file {
        Some(out_file) => {
            std::fs::write(out_file, data)?;
        }
        None => {
            std::io::stdout().write_all(data)?;
        }
    }
    Ok(())
}
