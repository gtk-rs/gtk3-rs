# gtk-rs

The `gtk-rs` organization aims to provide safe Rust binding over `GObject`-based libraries.
You can find more about it on <https://gtk-rs.org>.

This repository contains all the "core" crates of the gtk-rs organization. For more
information about each crate, please refer to their `README.md` file in their directory.

## Regenerating

To regenerate crates using [gir], please use the `generator.py`
file as follows:

```bash
$ python3 generator.py
```

## Development

This repository is structured as follows:

```text
- crate/
   |
   |-- README.md
   |-- Gir.toml
   |-- Cargo.toml
   |-- src/
   |-- sys/
```

The `crate` is a "top" directory (so "atk" or "gdk" in here for example). I listed some
import files, let's quickly explain them:

 * `README.md`: Explanations about the crate itself and eventually some details.
 * `Gir.toml`: Used by [gir] to generate most of the crates' code.
 * `Cargo.toml`: File describing the crate, used by `cargo` and `Rust`.
 * `src`: Contains the source code of the crate.
 * `sys`: Contains the 1:1 bindings of the C API.

[gir]: https://github.com/gtk-rs/gir
