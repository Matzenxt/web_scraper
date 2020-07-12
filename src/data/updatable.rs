use crate::data::plugin::Plugin;
use crate::data::version::Version;

#[derive(Default, Debug)]
pub struct Updatable {
    plugin: Plugin,
    version: Version,
}

impl Updatable {
    pub fn new(plugin: Plugin, version: Version) -> Updatable {
        Updatable {
            plugin,
            version
        }
    }

    pub fn print(&self) {
        println!("Update: {}", plugin.name);
        println!("    {} -> {}", plugin.version, scraped_version);
    }
}