mod cmd;
mod config;
mod fs;
mod gui;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let config = config::Config::open()?;

    gui::run(config)?;

    Ok(())
}
