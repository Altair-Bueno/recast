use tap::prelude::*;

use super::{Payload, Read, Result, Write};

pub fn from_reader(mut r: impl Read) -> Result<Payload> {
    let mut buff = Vec::new();
    r.read_to_end(&mut buff)?;
    toml::from_slice::<Payload>(&buff)?.pipe(Ok)
}

pub fn to_writer(mut w: impl Write, value: &Payload) -> Result<()> {
    toml::to_vec(value)?.pipe_ref(|x| w.write(x))?;
    Ok(())
}
