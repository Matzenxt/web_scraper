use crate::data::plugin::Plugin;
use crate::data::version::Version;
use std::fmt::{Formatter, Display, Result};

#[derive(Default, Debug)]
pub struct Updatable {
    pub plugin: Plugin,
    pub version: Version,
}

impl Updatable {
    pub fn new(plugin: Plugin, version: Version) -> Updatable {
        Updatable {
            plugin,
            version
        }
    }

    pub fn print(&self) {
        println!("Update: {}", self.plugin.name);
        println!("    {} -> {}", self.plugin.version, self.version);
    }
}

impl Display for Updatable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Update: {}\n        {} -> {}",
            self.plugin.name, self.plugin.version, self.version
        )
    }
}