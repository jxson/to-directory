MAKEFLAGS += --warn-undefined-variables
SHELL := /bin/bash
PATH := "$(shell pwd)/target/debug:${PATH}"

.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:
.SUFFIXES:

CARGO = $(shell which cargo)
CARGO_OPTS =

bats := "$(shell pwd)/deps/bats/bin/bats"

PHONY: all
all: build

PHONY: build
build:
	@$(CARGO) $(CARGO_OPTS) build

PHONY: test
test:
	@echo "== running: cargo test"
	@$(CARGO) $(CARGO_OPTS) test

# TODO(jxson): derive the directory correctly.
# http://stackoverflow.com/questions/18136918/how-to-get-current-relative-directory-of-your-makefile
PHONY: init
init: build # Run with: eval "$(make init)"
	@echo -e "export PATH=\"$(shell pwd)/target/debug:\$${PATH}\""
	@echo "$$(cargo run -q -- --init)"

PHONY: install
install:
	cargo build --release
	cp target/release/to-directory "$$(brew --prefix)/bin"

PHONY: fmt
fmt:
	@$(CARGO) $(CARGO_OPTS) fmt -- --write-mode overwrite
