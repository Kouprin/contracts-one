use regex::Regex;
use std::cmp::Ordering;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl From<&Version> for String {
    fn from(v: &Version) -> Self {
        format!("{}.{}.{}", v.major, v.minor, v.patch)
    }
}

impl From<&String> for Version {
    fn from(s: &String) -> Self {
        let re = Regex::new(r"^([1-9]+[0-9]*{1,6}|0).([1-9]+[0-9]*{1,6}|0).([1-9]+[0-9]*{1,8}|0)$")
            .unwrap();
        let captures = re.captures(&s).unwrap();
        assert_eq!(captures.len(), 4);
        Version {
            major: captures[1].parse::<u64>().unwrap(),
            minor: captures[2].parse::<u64>().unwrap(),
            patch: captures[3].parse::<u64>().unwrap(),
        }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        (self.major, self.minor, self.patch) == (other.major, other.minor, other.patch)
    }
}

impl Eq for Version {}
