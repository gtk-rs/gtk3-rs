// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{resources_register, Resource};
use glib::translate::*;
use std::env;
use std::mem;
use std::path::Path;
use std::process::Command;
use std::ptr;

impl Resource {
    #[doc(alias = "g_resource_new_from_data")]
    pub fn from_data(data: &glib::Bytes) -> Result<Resource, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();

            // Create a copy of data if it is not pointer-aligned
            // https://bugzilla.gnome.org/show_bug.cgi?id=790030
            let mut data = data.clone();
            let data_ptr = glib::ffi::g_bytes_get_data(data.to_glib_none().0, ptr::null_mut());
            if data_ptr as usize % mem::align_of::<*const u8>() != 0 {
                data = glib::Bytes::from(&*data);
            }

            let ret = ffi::g_resource_new_from_data(data.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// Call from build script to run `glib-compile-resources` to generate compiled gresources to embed
/// in binary with [resources_register_include]. `target` is relative to `OUT_DIR`.
///
/// ```no_run
/// gio::compile_resources(
///     "resources",
///     "resources/resources.gresource.xml",
///     "compiled.gresource",
/// );
/// ```
pub fn compile_resources<P: AsRef<Path>>(source_dir: P, gresource: &str, target: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new("glib-compile-resources")
        .arg("--sourcedir")
        .arg(source_dir.as_ref())
        .arg("--target")
        .arg(&format!("{}/{}", out_dir, target))
        .arg(gresource)
        .status()
        .unwrap();

    if !status.success() {
        panic!("glib-compile-resources failed with exit status {}", status);
    }

    println!("cargo:rerun-if-changed={}", gresource);
    let output = Command::new("glib-compile-resources")
        .arg("--sourcedir")
        .arg(source_dir.as_ref())
        .arg("--generate-dependencies")
        .arg(gresource)
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();
    for dep in output.split_whitespace() {
        println!("cargo:rerun-if-changed={}", dep);
    }
}

#[doc(hidden)]
pub fn resources_register_include_impl(bytes: &'static [u8]) -> Result<(), glib::Error> {
    let bytes = glib::Bytes::from_static(bytes);
    let resource = Resource::from_data(&bytes)?;
    resources_register(&resource);
    Ok(())
}

// rustdoc-stripper-ignore-next
/// Include gresources generated with [compile_resources] and register with glib. `path` is
/// relative to `OUTDIR`.
///
/// ```ignore
/// gio::resources_register_include!("compiled.gresource").unwrap();
/// ```
#[macro_export]
macro_rules! resources_register_include {
    ($path:expr) => {
        $crate::resources_register_include_impl(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/",
            $path
        )))
    };
}
