GIR = gir/target/release/gir
CONFIGS = $(wildcard conf/gir-*.toml)
LIBS = $(CONFIGS:conf/gir-%.toml=%-sys/src/lib.rs)

libs : $(LIBS)

%-sys/src/lib.rs : conf/gir-%.toml $(GIR)
	mkdir -p $(@D)
	$(GIR) -c $<

$(GIR) :
	cd gir && cargo build --release
