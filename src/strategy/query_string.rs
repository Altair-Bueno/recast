use super::{Payload, Read, Result, Write};
use tap::prelude::*;

pub fn from_reader(mut r: impl Read) -> Result<Payload> {
    let mut buff = Vec::new();
    r.read_to_end(&mut buff)?;
    serde_qs::from_bytes::<Payload>(&buff)?.pipe(Ok)
}

pub fn to_writer(mut w: impl Write, value: &Payload) -> Result<()> {
    serde_qs::to_writer(&value, &mut w)?.pipe(Ok)
}
