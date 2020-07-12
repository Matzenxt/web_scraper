use crate::data::version::Version;
use crate::data::plugin::Plugin;
use crate::data::registry::Registry;
use crate::data::updatable::Updatable;

mod data;
mod scraping;

fn main() {
    println!("Hello, world!");

    let registry: Registry = Registry::new("to_scrape_shopware.json".to_string());

    println!("{} plugins to check:", registry.plugins.len());
    for plugin in &registry.plugins {
        plugin.print_information();
    }

    let updatables: Vec<Updatable> = registry.check_for_updates();
    println!("\n{} plugins to update:", updatables.len());
    for updatable in updatables {
        println!("    - {}", updatable);
    }

    println!("End main");
}
