fn main() {
    manage_docs();
}

#[cfg(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))]
fn manage_docs () {
    extern crate lgpl_docs;
    const PATH: &'static str = "src";
    const IGNORES: &'static [&'static str] = &[];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::Library::Cairo, PATH, IGNORES);
    }
}

#[cfg(not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs")))]
fn manage_docs() { }
