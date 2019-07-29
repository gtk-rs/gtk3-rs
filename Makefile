GIR = gir/target/bin/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/Graphene-1.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs
	cargo fmt

doc: $(GIR) $(GIR_FILES)
	$(GIR) -m doc -c Gir.toml

not_bound: $(GIR) $(GIR_FILES)
	$(GIR) -m not_bound -c Gir.toml

regen_check: $(GIR) $(GIR_FILES)
	rm src/auto/*
	rm graphene-sys/tests/*.c graphene-sys/tests/*.rs
	$(GIR) -c Gir.toml
	$(GIR) -c graphene-sys/Gir.toml
	cargo fmt
	cd graphene-sys && cargo fmt
	git diff -R --exit-code

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

gir-sys: graphene-sys/src/lib.rs
	cd graphene-sys && cargo fmt

graphene-sys/src/lib.rs : graphene-sys/Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c graphene-sys/Gir.toml

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
