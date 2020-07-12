use crate::data::version::Version;
use crate::data::plugin::Plugin;
use crate::data::registry::Registry;
use crate::data::updatable::Updatable;

mod data;
mod scraping;

fn main() {
    println!("Hello, world!");

    let registry: Registry = Registry::new("to_scrape_shopware.json".to_string());

    for plugin in &registry.plugins {
        plugin.print_information();
    }

    println!("Plugins to update:");
    let updatables: Vec<Updatable> = registry.check_for_updates();
    for updatable in updatables {
        println!("    - {}", updatable);
    }

    let url = "https://store.shopware.com/media57848636557/facebook-pixel-einbinden.html";

    let version_string = String::from("1.6.6");
    let plugin: Plugin = Plugin::new("Test Plugin".to_string(), version_string, url.to_string());
    plugin.print_information();

    scraping::shopware::scrape_plugin(&plugin);

    println!("End main");
}
