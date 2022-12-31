use crate::cli::Format;
use crate::types::Payload;
use crate::types::Result;
use std::io::Read;
use std::io::Write;

mod csv;
mod query_string;
mod toml;

impl Format {
    pub fn load(self, r: impl Read) -> Result<Payload> {
        match self {
            Self::Toml => crate::strategy::toml::from_reader(r),
            Self::Yaml => crate::strategy::yaml::from_reader(r),
            Self::Json => crate::strategy::json::from_reader(r),
            Self::Query => crate::strategy::query_string::from_reader(r),
            Self::Csv => crate::strategy::csv::from_reader(r),
        }
    }

    pub fn dump(self, w: impl Write, value: &Payload) -> Result<()> {
        match self {
            Self::Toml => crate::strategy::toml::to_writer(w, value),
            Self::Yaml => crate::strategy::yaml::to_writer(w, value),
            Self::Json => crate::strategy::json::to_writer(w, value),
            Self::Query => crate::strategy::query_string::to_writer(w, value),
            Self::Csv => crate::strategy::csv::to_writer(w, value),
        }
    }
}

macro_rules! gen_strategy {
    ( $mod_name:tt, $from_reader_fn:expr, $to_writer_fn:expr ) => {
        mod $mod_name {
            use super::{Payload, Read, Result, Write};

            pub fn from_reader(r: impl Read) -> Result<Payload> {
                let r = $from_reader_fn(r)?;
                Ok(r)
            }

            pub fn to_writer(w: impl Write, value: &Payload) -> Result<()> {
                $to_writer_fn(w, value)?;
                Ok(())
            }
        }
    };
}

gen_strategy!(yaml, serde_yaml::from_reader, serde_yaml::to_writer);
gen_strategy!(json, serde_json::from_reader, serde_json::to_writer_pretty);
