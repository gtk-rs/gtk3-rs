# gtk-rs examples [![Build Status](https://travis-ci.org/gtk-rs/examples.png?branch=master)](https://travis-ci.org/gtk-rs/examples) [![Build status](https://ci.appveyor.com/api/projects/status/pi27a5xubp0ihl2d?svg=true)](https://ci.appveyor.com/project/GuillaumeGomez/examples)

A few gtk-rs examples. To build, just do:

```Shell
> cargo build
```

or to enable GTK 3.1x, 3.2x, all examples as well:

```Shell
> cargo build --features gtk_3_18
> cargo build --features gtk_3_22_30
> cargo build --all-features
```

And then run the executables with:

``` Shell
./target/debug/EXAMPLE-NAME
```
or with cargo run (repeating the compilation features used above), example:
``` Shell
cargo run --all-features --bin EXAMPLE-NAME
```

Please be sure to have installed all the required libraries before building examples (the list is available on [gtk](https://github.com/gtk-rs/gtk/).

## LICENSE
The gtk-rs examples repository is licensed under the MIT license, please refer to the LICENSE and COPYRIGHT files for more information.
