use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell;

use std::error::Error;

include!("src/cli.rs");

const COMPLETITIONS_DIR: &str = "target/completitions";

fn gen_completitons() -> Result<(), Box<dyn Error>> {
    _ = std::fs::create_dir(COMPLETITIONS_DIR);
    let mut cmd = Config::command();
    for shell in Shell::value_variants() {
        let path = generate_to(*shell, &mut cmd, "recast", COMPLETITIONS_DIR)?;
        println!("cargo:warning=RECAST_{shell}={path:?}");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let res = gen_completitons();
    println!("cargo:warning=RECAST_GENCOMPL={res:?}");

    Ok(())
}
