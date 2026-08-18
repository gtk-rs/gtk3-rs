fn main() {
    check_features();
}

fn check_features() {
    const BACKENDS: [&str; 5] = ["x11", "win32", "quartz", "broadway", "wayland"];

    let values = BACKENDS.map(|backend| format!("\"{backend}\"")).join(", ");
    println!("cargo:rustc-check-cfg=cfg(gdk_backend, values({values}))");

    if std::env::var("DOCS_RS").is_ok() {
        // There is no GTK to probe when building documentation, so assume every
        // backend is present rather than documenting none of the gated API.
        for backend in BACKENDS {
            println!("cargo:rustc-cfg=gdk_backend=\"{backend}\"");
        }
    } else {
        // The pkg-config file defines a `targets` variable listing the
        // various backends that gdk (yes, gdk) was compiled for.
        // We extract that and create gdk_backend="x11" and the like
        // as configuration variables.
        if let Ok(targets) = pkg_config::get_variable("gtk+-3.0", "targets") {
            for target in targets.split_whitespace() {
                println!("cargo:rustc-cfg=gdk_backend=\"{target}\"");
            }
        }
    }
}
