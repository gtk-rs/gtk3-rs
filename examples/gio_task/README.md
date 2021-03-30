# GTask example

This example demonstrates how to implement a GIO async/finish API based on GTask (see
https://developer.gnome.org/gio/stable/GTask.html) in Rust using the GTask generated bindings.
This can be useful, for example, when porting to Rust some existing C code exposing such an API.

Run it by executing:

```bash
cargo run --bin gio_futures_await
```
