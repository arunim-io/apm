mod gui;

use color_eyre::eyre::Result;
use gui::run_gui;

fn main() -> Result<()> {
    color_eyre::install()?;

    run_gui()?;

    Ok(())
}
