use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub buttons: Vec<Button>,
    pub spacing: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Button {
    pub label: String,
    pub cmd: String,
    pub icon: String,
}
