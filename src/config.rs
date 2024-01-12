use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub buttons: Vec<Button>,
    pub spacing: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Button {
    pub icon: String,
    pub label: String,
    pub cmd: Option<String>,
    pub key: Option<String>,
}
