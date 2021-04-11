# atk 
[Project site](https://gtk-rs.org/) | [Online documentation](https://gtk-rs.org/docs-src/)

__Rust__ bindings and wrappers for __Atk__, part of [gtk-rs](https://github.com/gtk-rs/gtk-rs).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

 * [Rust API - Stable](https://gtk-rs.org/docs/atk/)
 * [Rust API - Development](https://gtk-rs.org/gtk-rs/git/docs/atk)
 * [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
atk = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "atk" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
atk = "0.13"
atk = { git = "https://github.com/gtk-rs/gtk-rs.git", package = "atk" }

### See Also

 * [glib](https://crates.io/crates/glib)

## License

__atk__ is available under the MIT License, please refer to it.
