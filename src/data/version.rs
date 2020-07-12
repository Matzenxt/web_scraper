use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl Version {
    pub fn new(version_string: String) -> Version {
        let test: Vec<&str> = version_string.split('.').collect::<Vec<&str>>();

        Version {
            major: test[0].trim().parse::<usize>().unwrap(),
            minor: test[1].trim().parse::<usize>().unwrap(),
            patch: test[2].trim().parse::<usize>().unwrap(),
        }
    }

    pub fn print(&self) {
        println!("Version: {}.{}.{}", self.major, self.minor, self.patch);
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Version: {}.{}.{}", self.major, self.minor, self.patch)
    }
}
