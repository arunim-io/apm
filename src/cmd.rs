use std::process::Command;

use crate::config::Button;

impl Button {
    pub fn exec_cmd(self) {
        Command::new("sh")
            .args(["-c", &self.cmd])
            .output()
            .expect("Unable to run command");
        std::process::exit(0);
    }
}
