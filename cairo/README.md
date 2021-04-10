# cairo 

__Rust__ bindings for Rust and wrappers for __Cairo__.

![screenshot](https://guillaume-gomez.fr/image/cairo.png)

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

 * [Rust API - Stable](https://gtk-rs.org/docs/cairo)
 * [Rust API - Development](https://gtk-rs.org/gtk-rs/git/docs/cairo)
 * [C API](https://www.cairographics.org/documentation/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
cairo-rs = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "cairo-rs" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
cairo-rs = "0.13"
cairo-rs = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "cairo-rs" }

### See Also

 * [glib](https://crates.io/crates/glib)

## License

__cairo__ is available under the MIT License, please refer to it.
