use super::{Payload, Read, Result, Write};
use color_eyre::{Help, SectionExt};
use eyre::eyre;
use itertools::Itertools;

pub fn from_reader(r: impl Read) -> Result<Payload> {
    let mut reader = csv::Reader::from_reader(r);
    let mut vec = Vec::new();
    for e in reader.deserialize() {
        let e = Payload::Table(e?);
        vec.push(e);
    }
    Ok(Payload::Array(vec))
}

pub fn to_writer(w: impl Write, value: &Payload) -> Result<()> {
    let value = value
        .as_array()
        .ok_or_else(|| eyre!("Payload is not an array"))?;

    // Get inner objects and their keys
    let objects: Vec<_> = value.iter().filter_map(Payload::as_table).collect();
    let keys: Vec<_> = objects.iter().flat_map(|x| x.keys()).unique().collect();

    let mut writer = csv::Writer::from_writer(w);

    // Write header
    writer.write_record(&keys)?;
    for e in objects {
        let mut values = Vec::new();
        for key in &keys {
            let value = e.get(key.as_str());
            let value = if let Some(value) = value {
                match value {
                    Payload::Boolean(x) => x.to_string(),
                    Payload::Float(x) => x.to_string(),
                    Payload::Datetime(x) => x.to_string(),
                    Payload::Integer(x) => x.to_string(),
                    Payload::String(x) => x.to_string(),
                    x => Err(eyre!("Cannot serialize payload as csv"))?,
                }
            } else {
                String::default()
            };
            values.push(value);
        }
        writer.write_record(values)?;
    }
    Ok(())
}
