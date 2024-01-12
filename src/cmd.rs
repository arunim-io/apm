use std::process::Command;

use color_eyre::eyre::Context;

use crate::config::Button;

impl Button {
    pub fn exec_cmd(&self) {
        if let Some(cmd) = &self.cmd {
            let _ = Command::new("sh")
                .args(["-c", cmd])
                .output()
                .context("Unable to run command");
        }
        std::process::exit(0);
    }
}
