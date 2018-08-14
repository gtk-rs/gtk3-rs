extern crate pkg_config;

use pkg_config::{Config, Error};
use std::env;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    if cfg!(feature = "use_glib") {
        // This include cairo linker flags
        if let Err(s) = find("cairo-gobject") {
            let _ = writeln!(io::stderr(), "{}", s);
            process::exit(1);
        }
    } else {
        if let Err(s) = find("cairo") {
            let _ = writeln!(io::stderr(), "{}", s);
            process::exit(1);
        }
    }
}

fn find(name: &str) -> Result<(), Error> {
    let package_name = name;
    let shared_libs = [name];
    let version = if cfg!(feature = "1.14") {
        "1.14"
    } else if cfg!(feature = "1.12") {
        "1.12"
    } else {
        "1.10"
    };

    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return Ok(());
    }

    let target = env::var("TARGET").unwrap();
    let hardcode_shared_libs = target.contains("windows");

    let mut config = Config::new();
    config.atleast_version(version);
    config.print_system_libs(false);

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
