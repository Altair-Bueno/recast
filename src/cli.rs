use camino::Utf8PathBuf;
use clap::Parser;
use clap::ValueEnum;

/// Command line arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "recast",author, version, about, long_about = None)]
pub struct Config {
    /// Input format
    #[arg(short, long, default_value_t, value_name = "FORMAT")]
    pub from: Format,
    /// Output format
    #[arg(short, long, default_value_t, value_name = "FORMAT")]
    pub to: Format,
    /// Output to file
    ///
    /// If no file is provided, recast will default to STDOUT
    #[arg(short, long, value_name = "FILE")]
    pub out: Option<Utf8PathBuf>,
    /// Input from file
    ///
    /// If no file is provided, recast will default to STDIN
    #[arg(verbatim_doc_comment)]
    pub file: Option<Utf8PathBuf>,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum, enum_display::EnumDisplay)]
#[enum_display(case = "Kebab")]
pub enum Format {
    #[default]
    Json,
    Toml,
    Yaml,
    Query,
    Csv,
    Xml,
}
