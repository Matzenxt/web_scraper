use std::fs;
use crate::data::plugin::Plugin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Registry {
    pub plugins: Vec<Plugin>,
}

impl Registry {
    pub fn new(path: String) -> Registry {
        // Load file
        let content = fs::read_to_string(path).expect("Error while reading file");

        // Parse json
        let reg: Registry = serde_json::from_str(&content.as_str()).unwrap();

        reg
    }
}