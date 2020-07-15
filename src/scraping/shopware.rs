use scraper::{Html, Selector};

use crate::data::plugin::Plugin;
use crate::data::version::Version;

pub fn scrape_plugin(plugin: &Plugin) -> Version {
    let resp = reqwest::blocking::get(plugin.url.as_str()).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();

    let html: Html = Html::parse_document(&body);
    let selector_version_label: Selector = Selector::parse(".entry--label").unwrap();
    let selector_version_number: Selector = Selector::parse(".entry--content").unwrap();

    // Find position of label for version
    let mut position: usize = 0;
    for version_label in html.select(&selector_version_label).enumerate() {
        let temp_text: String = version_label.1.text().collect();

        if temp_text.contains(&"Version") {
            position = version_label.0;
            break;
        }
    }

    // Get version as string from webpage
    let mut version_string: String = String::from("0.0.0");
    for version_number in html.select(&selector_version_number).enumerate() {
        if version_number.0 == position {
            let temp_text: String = version_number.1.text().collect();
            version_string = temp_text;
        }
    }

    // Creat and return version
    let scraped_version: Version = Version::new(version_string);
    scraped_version
}

// TODO: impl scraper for system version
pub fn scrape_system_version() {

}