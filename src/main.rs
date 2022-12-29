#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations
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
use std::io::*;

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
