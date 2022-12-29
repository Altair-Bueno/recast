use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell;

use std::env;
use std::error::Error;

include!("src/cli.rs");

fn gen_completitons() -> Result<(), Box<dyn Error>> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let mut cmd = Config::command();
    for shell in Shell::value_variants() {
        let path = generate_to(*shell, &mut cmd, "recast", outdir.clone())?;
        // cargo build 2>&1 >/dev/null | sed -En 's/^.*(RECAST.+)="(.*)"$/\1=\2/gp'
        // RECAST_bash=/path/to/file
        println!("cargo:warning=RECAST_{shell}={path:?}");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    gen_completitons()?;

    Ok(())
}
