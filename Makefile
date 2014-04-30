RUSTC ?= rustc
RUSTC_FLAGS ?=

SRC = $(shell find src -name '*.rs')

all: libgossip

libgossip: $(SRC)
	mkdir -p target
	$(RUSTC) --out-dir target src/gossip/lib.rs

test: $(SRC)
	mkdir -p target
	$(RUSTC) --test --out-dir target src/gossip/lib.rs
	./target/gossip

clean:
	@rm -rf target


.PHONY: clean all
