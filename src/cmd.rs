use std::process::Command;

use color_eyre::eyre::WrapErr;

use crate::config::Button;

impl Button {
    pub fn exec_cmd(&self) {
        if let Some(cmd) = &self.cmd {
            let _ = Command::new("sh")
                .args(["-c", cmd])
                .output()
                .wrap_err("Unable to run command");
        }
        std::process::exit(0);
    }
}
