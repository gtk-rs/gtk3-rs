extern crate pkg_config;

use pkg_config::{Config, Error};
use std::env;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    if cfg!(feature = "use_glib") {
        // This include cairo linker flags
        if let Err(s) = find("cairo-gobject", &["cairo", "cairo-gobject"]) {
            let _ = writeln!(io::stderr(), "{}", s);
            process::exit(1);
        }
    } else {
        if let Err(s) = find("cairo", &["cairo"]) {
            let _ = writeln!(io::stderr(), "{}", s);
            process::exit(1);
        }
    }
}

fn find(package_name: &str, shared_libs: &[&str]) -> Result<(), Error> {
    let version = "1.12";

    if let Ok(inc_dir) = env::var("GTK_INCLUDE_DIR") {
        println!("cargo:include={}", inc_dir);
    }
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
            if let Ok(paths) = std::env::join_paths(library.include_paths) {
                // Exposed to other build scripts as DEP_CAIRO_INCLUDE; use env::split_paths
                println!("cargo:include={}", paths.to_string_lossy());
            }
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
