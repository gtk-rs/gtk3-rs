extern crate pkg_config;

const MIN_MAJOR: u16 = 1;
const MIN_MINOR: u16 = 10;
const MINOR_STEP: u16 = 2;

fn main() {
    let lib = pkg_config::find_library("cairo")
        .unwrap_or_else(|e| panic!("{}", e));
    let mut parts = lib.version.splitn(3, '.')
        .map(|s| s.parse())
        .take_while(|r| r.is_ok())
        .map(|r| r.unwrap());
    let version: (u16, u16) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    let mut cfgs = Vec::new();
    if version.0 == MIN_MAJOR && version.1 > MIN_MINOR {
        let major = version.0;
        let mut minor = MIN_MINOR + MINOR_STEP;
        while minor <= version.1 {
            cfgs.push(format!("cairo_{}_{}", major, minor));
            minor += MINOR_STEP;
        }
    }
    for cfg in &cfgs {
        println!("cargo:rustc-cfg={}", cfg);
    }
    println!("cargo:cfg={}", cfgs.connect(" "));
}
