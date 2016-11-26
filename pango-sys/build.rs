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
    let package_name = "pango";
    let shared_libs = ["pango-1.0"];
    let version = if cfg!(feature = "v1_42") {
        "1.42"
    } else if cfg!(feature = "v1_38") {
        "1.38"
    } else if cfg!(feature = "v1_36_7") {
        "1.36.7"
    } else if cfg!(feature = "v1_34") {
        "1.34"
    } else if cfg!(feature = "v1_32_4") {
        "1.32.4"
    } else if cfg!(feature = "v1_32") {
        "1.32"
    } else if cfg!(feature = "v1_31") {
        "1.31"
    } else {
        "1.30"
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

