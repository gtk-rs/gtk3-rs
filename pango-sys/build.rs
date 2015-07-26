
extern crate pkg_config;
use std::cmp::Ordering;

const LIBRARY_NAME: &'static str = "pango";
const PACKAGE_NAME: &'static str = "pango";
const VERSIONS: &'static [Version] = &[
	Version(1, 30, 0),
	Version(1, 31, 0),
	Version(1, 32, 0),
	Version(1, 32, 4),
	Version(1, 34, 0),
];

fn main() {
    let lib = pkg_config::find_library(PACKAGE_NAME)
        .unwrap_or_else(|e| panic!("{}", e));
    let version = Version::new(&lib.version);
    let mut cfgs = Vec::new();
    for v in VERSIONS.iter().filter(|&&v| v <= version) {
        let cfg = v.to_cfg();
        println!("cargo:rustc-cfg={}", cfg);
        cfgs.push(cfg);
    }
    println!("cargo:cfg={}", cfgs.connect(" "));
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Version(pub u16, pub u16, pub u16);

impl Version {
    fn new(s: &str) -> Version {
        let mut parts = s.splitn(4, '.')
            .map(|s| s.parse())
            .take_while(Result::is_ok)
            .map(Result::unwrap);
        Version(parts.next().unwrap_or(0),
            parts.next().unwrap_or(0), parts.next().unwrap_or(0))
    }

    fn to_cfg(&self) -> String {
        match *self {
            Version(major, minor, 0) => format!("{}_{}_{}", LIBRARY_NAME, major, minor),
            Version(major, minor, patch) =>
                format!("{}_{}_{}_{}", LIBRARY_NAME, major, minor, patch),
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => {
                match self.1.cmp(&other.1) {
                    Ordering::Equal => self.2.cmp(&other.2),
                    x => x,
                }
            }
            x => x,
        }
    }
}

