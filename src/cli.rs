use camino::Utf8PathBuf;
use clap::Parser;
use clap::ValueEnum;

/// Command line arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "recast",author, version, about, long_about = None)]
pub struct Config {
    /// Input format
    ///
    /// If no format is provided, recast will attempt to infer the format from the file extension
    #[arg(short, long, value_name = "FORMAT")]
    pub from: Option<Format>,
    /// Output format
    ///
    /// If no format is provided, recast will attempt to infer the format from the file extension
    #[arg(short, long, value_name = "FORMAT")]
    pub to: Option<Format>,
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

#[derive(Debug, Clone, Copy, ValueEnum, enum_display::EnumDisplay)]
#[enum_display(case = "Kebab")]
pub enum Format {
    Json,
    Toml,
    Yaml,
    Query,
    Csv,
    Xml,
    Msgpack,
}

impl Format {
    pub fn from_extension(s: &str) -> Option<Self> {
        let format = match s {
            "json" => Self::Json,
            "toml" => Self::Toml,
            "yaml" | "yml" => Self::Yaml,
            "qs" => Self::Query,
            "csv" => Self::Csv,
            "xml" => Self::Xml,
            "msgpack" | "mpk" => Self::Msgpack,
            _ => return None,
        };
        Some(format)
    }
}
