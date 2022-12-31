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
