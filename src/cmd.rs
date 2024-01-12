use std::process::Command;

use crate::config::Button;

impl Button {
    pub fn exec_cmd(self) {
        if let Some(cmd) = self.cmd {
            Command::new("sh")
                .args(["-c", &cmd])
                .output()
                .expect("Unable to run command");
            std::process::exit(0);
        }
    }
}
