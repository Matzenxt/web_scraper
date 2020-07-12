use std::fs;
use crate::data::plugin::Plugin;
use crate::data::updatable::Updatable;
use serde::{Deserialize, Serialize};
use crate::scraping;
use crate::data::version::Version;

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

    pub fn check_for_updates(&self) -> Vec<Updatable> {
        let mut result: Vec<Updatable> = Vec::new();

        for plugin in &self.plugins {
            //let temp: Plugin = plugin.clone();
            let scraped_version: Version = scraping::shopware::scrape_plugin(&plugin);

            if plugin.version < scraped_version {
                result.push(Updatable {
                    plugin: plugin.clone(),
                    version: scraped_version
                });
            }
        }

        result
    }
}