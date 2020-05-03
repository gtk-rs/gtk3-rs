#[cfg(not(feature = "dox"))]
extern crate pkg_config;

#[cfg(not(feature = "dox"))]
use pkg_config::{Config, Error};
#[cfg(not(feature = "dox"))]
use std::env;
#[cfg(not(feature = "dox"))]
use std::io;
#[cfg(not(feature = "dox"))]
use std::io::prelude::*;
#[cfg(not(feature = "dox"))]
use std::process;

mod build_version;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
    // It's safe to assume we can call this because we found the library OK
    // in find()
    check_features();
}

#[cfg(not(feature = "dox"))]
const PKG_CONFIG_PACKAGE: &str = "gdk-3.0";

#[cfg(not(feature = "dox"))]
fn find() -> Result<(), Error> {
    let package_name = PKG_CONFIG_PACKAGE;
    let shared_libs = ["gdk-3"];
    let version = build_version::version();

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

    let target = env::var("TARGET").expect("TARGET environment variable doesn't exist");
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
                println!("cargo:include={}", paths.to_string_lossy());
            }
            if hardcode_shared_libs {
                for lib_ in shared_libs.iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib_);
                }
                for path in library.link_paths.iter() {
                    println!(
                        "cargo:rustc-link-search=native={}",
                        path.to_str().expect("library path doesn't exist")
                    );
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

#[cfg(not(feature = "dox"))]
fn check_features() {
    // The pkg-config file defines a `targets` variable listing the
    // various backends that gdk was compiled for.
    // We extract that and create gdk_backend="x11" and the like
    // as configuration variables.
    // For reference, the backend set at time of writing consists of:
    // x11 win32 quartz broadway wayland
    if let Ok(targets) = pkg_config::get_variable(PKG_CONFIG_PACKAGE, "targets") {
        println!("cargo:backends={}", targets);
        for target in targets.split_whitespace() {
            println!("cargo:rustc-cfg=gdk_backend=\"{}\"", target);
        }
    }
}
