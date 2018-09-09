# Determine this makefile's path.
# Be sure to place this BEFORE `include` directives, if any.
THIS_FILE := $(lastword $(MAKEFILE_LIST))

GIR = gir/target/release/gir
CONFIGS = $(wildcard conf/gir-*.toml)
GIR_FILES = $(shell ls $(CONFIGS) | xargs -n1 perl -ne '$$name = $$1 if /^library = "(.*)"$$/; $$ver = $$1 if /^version = "(.*)"/; END { print "gir-files/$$name-$$ver.gir\n" }')
LIBS = $(CONFIGS:conf/gir-%.toml=%-sys/src/lib.rs)
TEST_C_FILES = $(CONFIGS:conf/gir-%.toml=%-sys/tests/*.c)
TEST_RS_FILES = $(CONFIGS:conf/gir-%.toml=%-sys/tests/*.rs)

libs : $(LIBS)

%-sys/src/lib.rs : conf/gir-%.toml $(GIR) $(GIR_FILES)
	$(GIR) -c $< -o $(abspath $*-sys) -d gir-files

$(GIR) : gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
	cd gir && cargo build --release

gir/Cargo.toml $(GIR_FILES) :
	git submodule update --init

regen_check: $(GIR) $(GIR_FILES)
	rm -f $(TEST_C_FILES)
	rm -f $(TEST_RS_FILES)
	@$(MAKE) -f $(THIS_FILE) libs
	git diff -R --exit-code
