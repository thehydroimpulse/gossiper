RUSTC ?= rustc
RUSTC_FLAGS ?=
CARGO ?= cargo

SRC = $(shell find src -name '*.rs')

all: libgossip

libgossip: $(SRC)
	$(CARGO) build

test: $(SRC)
	sh ./test/check-style.sh
	$(CARGO) test

clean:
	@rm -rf target

.PHONY: clean
