use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub buttons: Vec<Button>,
    pub spacing: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Button {
    pub label: Option<String>,
    pub cmd: Option<String>,
    pub icon: String,
}
