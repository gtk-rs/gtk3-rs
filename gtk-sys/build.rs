extern crate pkg_config;

use pkg_config::{Config, Error};
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

fn find() -> Result<(), Error> {
    let package_name = "gtk+-3.0";
    let shared_libs = ["gtk-3", "gdk-3"];
    let version = if cfg!(feature = "v3_22") {
        "3.22"
    } else if cfg!(feature = "v3_20") {
        "3.20"
    } else if cfg!(feature = "v3_18") {
        "3.18"
    } else if cfg!(feature = "v3_16") {
        "3.16"
    } else if cfg!(feature = "v3_14") {
        "3.14"
    } else if cfg!(feature = "v3_12") {
        "3.12"
    } else if cfg!(feature = "v3_10") {
        "3.10"
    } else if cfg!(feature = "v3_8") {
        "3.8"
    } else if cfg!(feature = "v3_6") {
        "3.6"
    } else {
        "3.4"
    };

    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return Ok(())
    }

    let target = env::var("TARGET").unwrap();
    let hardcode_shared_libs = target.contains("windows");

    let mut config = Config::new();
    config.atleast_version(version);
    if hardcode_shared_libs {
        config.cargo_metadata(false);
    }
    match config.probe(package_name) {
        Ok(library) => {
            if hardcode_shared_libs {
                for lib_ in shared_libs.iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib_);
                }
                for path in library.link_paths.iter() {
                    println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
                }
            }
            Ok(())
        }
        Err(Error::EnvNoPkgConfig(_)) | Err(Error::Command { .. }) => {
            for lib_ in shared_libs.iter() {
                println!("cargo:rustc-link-lib=dylib={}", lib_);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

