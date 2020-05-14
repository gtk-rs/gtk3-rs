fn main() {
    manage_docs();
}

#[cfg(all(
    any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"),
    not(all(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))
))]
fn manage_docs() {
    extern crate lgpl_docs;
    const PATH: &str = "src";
    const IGNORES: &[&str] = &["lib.rs", "prelude.rs"];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::Library::Gio, PATH, IGNORES);
    }
}

#[cfg(any(
    all(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"),
    not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))
))]
fn manage_docs() {}
