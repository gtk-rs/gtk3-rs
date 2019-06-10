GIR = gir/target/bin/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/Glib-2.0.gir gir-files/GObject-2.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs src/gobject/auto/mod.rs
	cargo fmt

doc: $(GIR) $(GIR_FILES)
	$(GIR) -m doc -c Gir.toml

not_bound: $(GIR) $(GIR_FILES)
	$(GIR) -m not_bound -c Gir.toml

regen_check: $(GIR) $(GIR_FILES)
	rm src/auto/*
	rm src/gobject/auto/*
	$(GIR) -c Gir.toml
	$(GIR) -c Gir_GObject.toml
	cargo fmt
	git diff -R --exit-code

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

src/gobject/auto/mod.rs : Gir_GObject.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir_GObject.toml

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
