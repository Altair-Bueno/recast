use crate::cli::Format;
use crate::types::*;
use std::io::Read;
use std::io::Write;

impl Format {
    pub fn load(&self, r: impl Read) -> Result<Payload> {
        match self {
            Format::Toml => crate::strategy::toml::from_reader(r),
            Format::Yaml => crate::strategy::yaml::from_reader(r),
            Format::Json => crate::strategy::json::from_reader(r),
            Format::Query => crate::strategy::query_string::from_reader(r),
            Format::Csv => crate::strategy::csv::from_reader(r),
        }
    }

    pub fn dump(&self, w: impl Write, value: &Payload) -> Result<()> {
        match self {
            Format::Toml => crate::strategy::toml::to_writer(w, value),
            Format::Yaml => crate::strategy::yaml::to_writer(w, value),
            Format::Json => crate::strategy::json::to_writer(w, value),
            Format::Query => crate::strategy::query_string::to_writer(w, value),
            Format::Csv => crate::strategy::csv::to_writer(w, value),
        }
    }
}

mod toml {
    use super::{Payload, Read, Result, Write};

    pub fn from_reader(mut r: impl Read) -> Result<Payload> {
        let mut buff = Vec::new();
        r.read_to_end(&mut buff)?;
        let res = toml::from_slice(&buff)?;
        Ok(res)
    }

    pub fn to_writer(mut w: impl Write, value: &Payload) -> Result<()> {
        let v = toml::to_vec(value)?;
        _ = w.write(&v)?;
        Ok(())
    }
}
mod query_string {
    use super::*;

    pub fn from_reader(mut r: impl Read) -> Result<Payload> {
        let mut buff = Vec::new();
        r.read_to_end(&mut buff)?;
        let res = serde_qs::from_bytes(&buff)?;
        Ok(res)
    }

    pub fn to_writer(mut w: impl Write, value: &Payload) -> Result<()> {
        serde_qs::to_writer(&value, &mut w)?;
        Ok(())
    }
}

mod csv {
    use super::{Payload, Read, Result, Write};
    use eyre::eyre;
    use itertools::Itertools;

    pub fn from_reader(r: impl Read) -> Result<Payload> {
        let mut reader = csv::Reader::from_reader(r);
        let mut vec = Vec::new();
        for e in reader.deserialize() {
            let e = Payload::Object(e?);
            vec.push(e);
        }
        Ok(Payload::Array(vec))
    }

    pub fn to_writer(w: impl Write, value: &Payload) -> Result<()> {
        let value = value
            .as_array()
            .ok_or_else(|| eyre!("Payload is not an array"))?;

        // Get inner objects and their keys
        let objects: Vec<_> = value.iter().flat_map(Payload::as_object).collect();
        let keys: Vec<_> = objects.iter().flat_map(|x| x.keys()).unique().collect();

        let mut writer = csv::Writer::from_writer(w);

        // Write header
        writer.write_record(&keys)?;
        for e in objects {
            let mut values = Vec::new();
            for key in keys.iter() {
                let value = e.get(key.as_str());
                let value = if let Some(value) = value {
                    match value {
                        Payload::Null => Default::default(),
                        Payload::Bool(x) => x.to_string(),
                        Payload::Number(x) => x.to_string(),
                        Payload::String(x) => x.to_string(),
                        x => Err(eyre!("Cannot serialize as csv: {x:?}"))?,
                    }
                } else {
                    Default::default()
                };
                values.push(value)
            }
            writer.write_record(values)?;
        }
        Ok(())
    }
}

macro_rules! gen_strategy {
    ( $mod_name:tt, $from_reader_fn:expr, $to_writer_fn:expr ) => {
        mod $mod_name {
            use super::*;

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
