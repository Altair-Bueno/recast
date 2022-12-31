use super::{Payload, Read, Result, Write};

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
