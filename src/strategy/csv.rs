use super::{Payload, Read, Result, Write};
use eyre::eyre;
use itertools::Itertools;

pub fn from_reader(r: impl Read) -> Result<Payload> {
    let v = csv::Reader::from_reader(r)
        .deserialize()
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(Payload::Sequence(v))
}

pub fn to_writer(w: impl Write, value: &Payload) -> Result<()> {
    let value = value
        .as_sequence()
        .ok_or_else(|| eyre!("Payload is not an array"))?;

    // Get inner objects and their keys
    let objects: Vec<_> = value.iter().filter_map(Payload::as_mapping).collect();
    let keys: Vec<_> = objects
        .iter()
        .flat_map(|x| x.keys())
        .filter_map(Payload::as_str)
        .unique()
        .collect();

    let mut writer = csv::Writer::from_writer(w);
    let mut buf = Vec::new();
    // Write header
    writer.write_record(&keys)?;
    // Write rows
    for e in objects {
        for key in &keys {
            match e.get(key) {
                Some(Payload::String(x)) => write!(buf, "{x}")?,
                Some(Payload::Bool(x)) => write!(buf, "{x}")?,
                Some(Payload::Number(x)) => write!(buf, "{x}")?,
                _ => (),
            };
            writer.write_field(&buf)?;
            buf.clear();
        }
        writer.write_record(std::iter::empty::<&[u8]>())?;
    }
    Ok(())
}
