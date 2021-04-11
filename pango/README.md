# pango

__Rust__ bindings and wrappers for __Pango__, part of [gtk-rs](https://github.com/gtk-rs/gtk-rs).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

 * [Rust API - Stable](https://gtk-rs.org/docs/pango/)
 * [Rust API - Development](https://gtk-rs.org/gtk-rs/git/docs/pango)
 * [C API](https://developer.gnome.org/platform-overview/unstable/tech-pango.html.en)
 * [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
pango = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "pango" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
pango = "0.13"
pango = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "pango" }

### See Also

 * [glib](https://crates.io/crates/glib)

## License

__pango__ is available under the MIT License, please refer to it.
