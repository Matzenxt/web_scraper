#[derive(Default)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl Version {
    pub fn new(version_string: String) -> Version {
        let test: Vec<&str> = version_string.split('.').collect::<Vec<&str>>();
        Version {
            major: test[0].parse::<usize>().unwrap(),
            minor: test[1].parse::<usize>().unwrap(),
            patch: test[2].parse::<usize>().unwrap(),
        }
    }

    pub fn print(&self) {
        println!("Version: {}.{}.{}", self.major, self.minor, self.patch);
    }
}