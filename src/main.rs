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
use cli::Config;
use color_eyre::Help;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use tap::prelude::*;

fn run(
    Config {
        from,
        to,
        out,
        file,
    }: Config,
) -> Result<()> {
    let reader: Box<dyn Read> = if let Some(pathbuf) = file {
        File::open(&pathbuf).with_note(|| pathbuf)?.pipe(Box::new)
    } else {
        std::io::stdin().lock().pipe(Box::new)
    };
    let reader = BufReader::new(reader);

    let writer: Box<dyn Write> = if let Some(pathbuf) = out {
        File::create(&pathbuf).with_note(|| pathbuf)?.pipe(Box::new)
    } else {
        std::io::stdout().lock().pipe(Box::new)
    };
    let writer = BufWriter::new(writer);

    from.load(reader)
        .with_note(|| format!("cannot deserialize as {from}"))?
        .pipe_ref(|x| to.dump(writer, x))
        .with_note(|| format!("cannot serialize as {to}"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Config::parse().pipe(run)
}
