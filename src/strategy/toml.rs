use tap::prelude::*;

use super::{Payload, Read, Result, Write};

pub fn from_reader(mut r: impl Read) -> Result<Payload> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    toml::from_str::<Payload>(&s)?.pipe(Ok)
}

pub fn to_writer(mut w: impl Write, value: &Payload) -> Result<()> {
    toml::to_string_pretty(value)?
        .as_bytes()
        .pipe(|x| w.write(x))?;
    Ok(())
}
