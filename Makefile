GIR = gir/target/release/gir
CONFIGS = $(wildcard conf/gir-*.toml)
LIBS = $(CONFIGS:conf/gir-%.toml=%-sys/src/lib.rs)

libs : $(LIBS)

%-sys/src/lib.rs : conf/gir-%.toml $(GIR)
	$(GIR) -c $< -o $(abspath $*-sys) -m sys

$(GIR) : gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
	cd gir && cargo build --release

gir/Cargo.toml :
	git submodule update --init

update :
	git submodule update --init
