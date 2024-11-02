use super::{Payload, Read, Result, Write};

pub fn from_reader(r: impl Read) -> Result<Payload> {
    rmp_serde::from_read(r).map_err(Into::into)
}

pub fn to_writer(mut w: impl Write, value: &Payload) -> Result<()> {
    let v = rmp_serde::to_vec(value)?;
    w.write_all(&v).map_err(Into::into)
}
