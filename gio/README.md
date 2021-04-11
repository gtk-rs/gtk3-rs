# gio

__Rust__ bindings and wrappers for __GIO__, part of [gtk-rs](https://github.com/gtk-rs/gtk-rs).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

 * [Rust API - Stable](https://gtk-rs.org/docs/gio/)
 * [Rust API - Development](https://gtk-rs.org/gtk-rs/git/docs/gio)
 * [C API](https://developer.gnome.org/gio/stable/)
 * [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gio = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "gio" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gio = "0.13"
gio = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "gio" }

### See Also

 * [glib](https://crates.io/crates/glib)

## License

__gio__ is available under the MIT License, please refer to it.
