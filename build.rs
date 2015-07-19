use std::env;

fn main() {
    let cfgs = env::var("OVERRIDE_CAIRO_CFG")
        .or_else(|_| env::var("DEP_CAIRO_CFG"))
        .unwrap_or_else(|e| panic!("Failed to read `DEP_CAIRO_CFG`: {}", e));
    for cfg in cfgs.split(' ').filter(|s| !s.is_empty()) {
        println!("cargo:rustc-cfg={}", cfg);
    }
}
