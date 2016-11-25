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
    let package_name = "glib-2.0";
    let shared_libs = ["glib-2.0", "gobject-2.0"];
    let version = if cfg!(feature = "v2_50") {
        "2.50"
    } else if cfg!(feature = "v2_48") {
        "2.48"
    } else if cfg!(feature = "v2_46") {
        "2.46"
    } else if cfg!(feature = "v2_44") {
        "2.44"
    } else if cfg!(feature = "v2_40") {
        "2.40"
    } else if cfg!(feature = "v2_38") {
        "2.38"
    } else if cfg!(feature = "v2_36") {
        "2.36"
    } else if cfg!(feature = "v2_34") {
        "2.34"
    } else {
        "2.32"
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

