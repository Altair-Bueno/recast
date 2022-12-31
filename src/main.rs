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

fn run(
    Config {
        from,
        to,
        out,
        file,
    }: Config,
) -> Result<()> {
    let reader: Box<dyn Read> = if let Some(pathbuf) = file {
        let file = File::open(&pathbuf).with_note(|| pathbuf)?;
        Box::new(file) as _
    } else {
        let inlock = std::io::stdin().lock();
        Box::new(inlock) as _
    };
    let reader = BufReader::new(reader);

    let writer: Box<dyn Write> = if let Some(pathbuf) = out {
        let file = File::open(&pathbuf).with_note(|| pathbuf)?;
        Box::new(file) as _
    } else {
        let outlock = std::io::stdout().lock();
        Box::new(outlock) as _
    };
    let writer = BufWriter::new(writer);

    let payload = from
        .load(reader)
        .with_note(|| format!("cannot deserialize as {from}"))?;
    to.dump(writer, &payload)
        .with_note(|| format!("cannot serialize as {to}"))?;

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let config = Config::parse();
    run(config)
}
