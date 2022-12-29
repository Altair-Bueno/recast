use camino::Utf8PathBuf;
use clap::Parser;
use clap::ValueEnum;

/// Command line arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "recast",author, version, about, long_about = None)]
pub struct Config {
    /// Input format
    #[arg(short, long, default_value_t)]
    pub from: Format,
    /// Output format
    #[arg(short, long, default_value_t)]
    pub to: Format,

    /// Output file
    ///
    /// If no file is provided, recast will default to STDOUT
    #[arg(short, long)]
    pub out: Option<Utf8PathBuf>,
    /// File to transform
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
}
