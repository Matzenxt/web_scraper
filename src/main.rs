use scraper::{Html, Selector};
use reqwest;

fn main() {
    println!("Hello, world!");

    let url = "https://store.shopware.com/media57848636557/facebook-pixel-einbinden.html";

    test(url);

    println!("End main");

}

fn test(url: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    //println!("{:#?}", resp);

    let body = resp.text().unwrap();

    let html = Html::parse_document(&body);
    let version = Selector::parse(".entry--content").unwrap();
    let version_label = Selector::parse(".entry--label").unwrap();


    println!("\n\n");

    let mut temp_pos: usize = 0;
    println!("Labels:");
    for version_one in html.select(&version_label).enumerate() {
        let temp_text = version_one.1.text().collect::<Vec<_>>();

        if temp_text.contains(&"\nVersion\n") {
            println!("{:?}", temp_text);
            println!("Position: {}", version_one.0);
            temp_pos = version_one.0;
            break;
        }
    }

    println!("\n\n");

    println!("Content:");
    for version_one in html.select(&version).enumerate() {
        if version_one.0 == temp_pos {
            let temp_text = version_one.1.text().collect::<Vec<_>>();
            println!("{:?}", temp_text);
        }
    }
}
