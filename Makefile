GIR = gir/target/release/gir
CONFIGS = $(wildcard conf/gir-*.toml)
GIR_FILES = $(shell ls $(CONFIGS) | xargs -n1 perl -ne '$$name = $$1 if /^library = "(.*)"$$/; $$ver = $$1 if /^version = "(.*)"/; END { print "gir-files/$$name-$$ver.gir\n" }')
LIBS = $(CONFIGS:conf/gir-%.toml=%-sys/src/lib.rs)

libs : $(LIBS)

%-sys/src/lib.rs : conf/gir-%.toml $(GIR) $(GIR_FILES)
	$(GIR) -c $< -o $(abspath $*-sys) -d gir-files

$(GIR) : gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
	cd gir && cargo build --release

gir/Cargo.toml $(GIR_FILES) :
	git submodule update --init
