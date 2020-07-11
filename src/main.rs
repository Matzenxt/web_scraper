use scraper::{Html, Selector};
use reqwest;
use regex::Regex;

fn main() {
    println!("Hello, world!");

    let url = "https://store.shopware.com/media57848636557/facebook-pixel-einbinden.html";

    test(url);

    println!("End main");
}

fn test(url: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();

    let html: Html = Html::parse_document(&body);
    let selector_version_label: Selector = Selector::parse(".entry--label").unwrap();
    let selector_version_number: Selector = Selector::parse(".entry--content").unwrap();

    println!("\n\n");

    let mut position: usize = 0;
    println!("Labels:");
    for version_label in html.select(&selector_version_label).enumerate() {
        let temp_text: String = version_label.1.text().collect();

        if temp_text.contains(&"\nVersion\n") {
            println!("{}", temp_text);
            println!("Position: {}", version_label.0);
            position = version_label.0;
            break;
        }
    }

    println!("\n\n");

    println!("Content:");
    for version_number in html.select(&selector_version_number).enumerate() {
        if version_number.0 == position {
            let temp_text: String = version_number.1.text().collect();
            println!("{}", &temp_text);
        }
    }
}
