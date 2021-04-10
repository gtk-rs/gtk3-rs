# gtk-rs ![CI](https://github.com/gtk-rs/gtk-rs/workflows/CI/badge.svg)

The `gtk-rs` organization aims to provide safe Rust binding over `GObject`-based libraries.
You can find more about it on <https://gtk-rs.org>.

This repository contains all the "core" crates of the gtk-rs organization. For more
information about each crate, please refer to their `README.md` file in their directory.

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

You can see the all the released crates' documentation in
[one place](https://gtk-rs.org/docs-src/).

## Regenerating

To regenerate crates using [gir], please use the `generator.py`
file as follows:

```bash
$ python3 generator.py
```

## Development

This repository is mostly split into two branches: `master` and `crate`.
`master` contains the not yet released code and is where new developments
are happening. `crate` contains the last release source code and isn't supposed to
be updated.

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

The `crate` is a "top" directory (so "atk" or "gdk" in here for example).
Each crate contains:

 * `README.md`: explanations about the crate itself and eventually some details.
 * `Cargo.toml`: descriptor of the crate, used by `cargo` and `Rust`.
 * `Gir.toml`: configuration used by [gir] to generate most of the crates' code.
 * `src`: the source code of the crate.
 * `sys`: the 1:1 bindings of the C API.

The `gir` and `gir-files` top folders are not crates, but are git submodules
which respectively contain the [gir] tool and the [gir files] used by the generator.

When running `generator.py` the tool will automatically update these git
submodules and run the [gir] tool on the [gir files] to regenerate the code.

During development, it is useful to execute the generator with a different
version of the [gir] tool or of the [gir files], for instance to test if
the code generation is successful before submitting a pull request to update
one of the submodules. This can be done by specifying arguments to the
generator script, for instance, to run the generator on a local copy of the
gir files:

```bash
$ python3 generator.py --gir-files-directory ../gir-files/
```

See `python3 generator.py --help` for more details.


[gir]: https://github.com/gtk-rs/gir
[gir files]: https://github.com/gtk-rs/gir-files
