MAKEFLAGS += --warn-undefined-variables
SHELL := /bin/bash
PATH := "deps/bats/bin:target/debug:${PATH}"

.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:
.SUFFIXES:

CARGO = $(shell which cargo)
CARGO_OPTS =

PHONY: all
all: build

PHONY: build
build:
	@$(CARGO) $(CARGO_OPTS) build

PHONY: test
test: test-rust test-shell

PHONY: test
test-rust:
	@echo "== running: cargo test"
	@$(CARGO) $(CARGO_OPTS) test

PHONY: test-integration
test-shell: build deps/bats
	@echo "== running: bats tests/integration.bats"
	@bats "tests/integration.bats"

# TODO(jxson): derive the directory correctly.
# http://stackoverflow.com/questions/18136918/how-to-get-current-relative-directory-of-your-makefile
PHONY: init
init: build # Run with: eval "$(make init)"
	@echo -e "export PATH=\"$(shell pwd)/target/debug:\$${PATH}\""
	@echo "$$(cargo run -q -- --init)"
