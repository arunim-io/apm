use std::env::var;

use copy_to_output::copy_to_output;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=examples/*");

    let dir = var("PROFILE")?;

    copy_to_output("examples/config.toml", &dir)?;
    copy_to_output("examples/styles.css", &dir)?;
    copy_to_output("examples/icons", &dir)?;

    Ok(())
}
