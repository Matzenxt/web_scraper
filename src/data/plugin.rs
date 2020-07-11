use crate::data::version::Version;

#[derive(Default)]
pub struct Plugin {
    pub name: String,
    pub version: Version,
    url: String,
}

impl Plugin {
    pub fn new(name: String, version_string: String, url: String) -> Plugin {
        Plugin {
            name: name.to_string(),
            version: Version::new(version_string),
            url: url.to_string(),
        }
    }

    pub fn print_information(&self) {
        println!("Information:");
        println!("  - Name: {}", self.name);
        println!("  - Version: {}.{}.{}", self.version.major, self.version.minor, self.version.patch);
        println!("  - URL: {}", self.url);
    }
}
