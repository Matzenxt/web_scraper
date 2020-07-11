use crate::data::version::Version;
use crate::data::plugin::Plugin;

mod data;
mod scraping;

fn main() {
    println!("Hello, world!");

    let url = "https://store.shopware.com/media57848636557/facebook-pixel-einbinden.html";

    let version_string = String::from("1.6.6");
    let plugin: Plugin = Plugin::new("Test Plugin".to_string(), version_string, url.to_string());
    plugin.print_information();

    scraping::shopware::scrape_plugin(plugin);

    println!("End main");
}
