extern crate pkg_config;
use std::env;
use std::io::prelude::*;
use std::io;
use std::process;

fn main() {
    if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}

fn find() -> Result<(), String> {
    let package_name = "atk";
    let shared_libs = ["atk-1.0"];
    let expected_version =
        if cfg!(feature = "2.12") {
            "2.12"
        } else if cfg!(feature = "2.10") {
            "2.10"
        } else if cfg!(feature = "2.9.4") {
            "2.9.4"
        } else if cfg!(feature = "2.9.3") {
            "2.9.3"
        } else if cfg!(feature = "2.8") {
            "2.8"
        } else {
            "2.4"
        };

    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        Ok(())
    } else {
        let lib = try!(pkg_config::find_library(package_name));
        if Version::new(&lib.version) >= Version::new(expected_version) {
            Ok(())
        } else {
            Err(format!("Installed `{}` version `{}` lower than `{}` requested by cargo features",
                        package_name, lib.version, expected_version))
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
}

