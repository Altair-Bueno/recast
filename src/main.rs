#![warn(clippy::nursery)]
#![deny(
    clippy::perf,
    clippy::pedantic,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![forbid(unsafe_code)]

mod cli;
mod strategy;
mod types;

use crate::types::Result;
use clap::Parser;
use cli::{Config, Format};
use color_eyre::Help;
use eyre::eyre;
use std::fs::File;
use std::io::{Read, Write};
use tap::prelude::*;

fn run(
    Config {
        from,
        to,
        out,
        file,
    }: Config,
) -> Result<()> {
    let (from, reader): (_, Box<dyn Read>) = match (from, file) {
        (None, None) => {
            return Err(eyre!("Missing input format for stdin")
                .with_note(|| "try specifying the input format with --from to read from stdin")
                .with_note(|| "try specifying the input file"))
        }
        (None, Some(path)) => {
            let Some(extension) = path.extension() else {
                return Err(eyre!(
                    "input file does not have a file extension to infer format from"
                )
                .with_note(|| "try specifying the format with --from"));
            };
            let Some(from) = Format::from_extension(extension) else {
                return Err(eyre!("unknown input file extension: {extension}")
                    .with_note(|| "try specifying the format with --from"));
            };
            let reader = File::open(&path).with_note(|| path)?.pipe(Box::new);
            (from, reader)
        }
        (Some(from), None) => (from, std::io::stdin().lock().pipe(Box::new)),
        (Some(from), Some(path)) => {
            let reader = File::open(&path).with_note(|| path)?.pipe(Box::new);
            (from, reader)
        }
    };

    let (to, writer): (_, Box<dyn Write>) = match (to, out) {
        (None, None) => {
            return Err(eyre!("missing output format for stdout")
                .with_note(|| "try specifying the output format with --to to write to stdout")
                .with_note(|| "try specifying the output file with --out"))
        }
        (None, Some(path)) => {
            let Some(extension) = path.extension() else {
                return Err(
                    eyre!("output file does not have a file extension to infer format")
                        .with_note(|| "try specifying the format with --to"),
                );
            };
            let Some(from) = Format::from_extension(extension) else {
                return Err(eyre!("Unknown output file extension: {extension}"))
                    .with_note(|| "try specifying the output format with --to");
            };
            let reader = File::create(&path).with_note(|| path)?.pipe(Box::new);
            (from, reader)
        }
        (Some(from), None) => (from, std::io::stdout().lock().pipe(Box::new)),
        (Some(from), Some(path)) => {
            let reader = File::create(&path).with_note(|| path)?.pipe(Box::new);
            (from, reader)
        }
    };

    from.load(reader)
        .with_note(|| format!("cannot deserialize as {from}"))?
        .pipe_ref(|x| to.dump(writer, x))
        .with_note(|| format!("cannot serialize as {to}"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Config::parse().pipe(run)
}
