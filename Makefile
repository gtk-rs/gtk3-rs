GIR = gir/target/release/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/Gio-2.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

$(GIR) : $(GIR_SRC)
	cd gir && cargo build --release

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
