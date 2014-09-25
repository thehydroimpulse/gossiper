RUSTC ?= rustc
RUSTC_FLAGS ?=
CARGO ?= cargo

SRC = $(shell find src -name '*.rs')

all: libgossip

libgossip: $(SRC)
	$(CARGO) build

sh: tests/compile.sh
	chmod +x tests/compile.sh

test: sh
	sh ./tests/check-style.sh
	./tests/compile.sh
	$(CARGO) test

clean:
	@rm -rf target

.PHONY: clean test
