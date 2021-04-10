# pangocairo 

__Rust__ bindings and wrappers for __PangoCairo__, part of [gtk-rs](https://github.com/gtk-rs/gtk-rs).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

 * [Rust API - Stable](https://gtk-rs.org/docs/pangocairo/)
 * [Rust API - Development](https://gtk-rs.org/gtk-rs/git/docs/pangocairo)
 * [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
pangocairo = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "pangocairo" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
pangocairo = "0.13"
pangocairo = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "pangocairo" }

### See Also

 * [cairo](https://crates.io/crates/cairo-rs)
 * [glib](https://crates.io/crates/glib)
 * [pango](https://crates.io/crates/pango)

## License

__pangocairo__ is available under the MIT License, please refer to it.
