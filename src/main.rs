mod cmd;
mod config;
mod fs;
mod gui;

use config::Config;

fn main() {
    let config = Config::open();

    gui::run(config);
}
