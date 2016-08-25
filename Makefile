MAKEFLAGS += --warn-undefined-variables
SHELL := /bin/bash

.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:
.SUFFIXES:

CARGO = cargo
CARGO_OPTS =

PHONY: build
build:
	$(CARGO) $(CARGO_OPTS) build

PHONY: install
install:
	$(CARGO) $(CARGO_OPTS) build --release

PHONY: test
test:
	$(CARGO) $(CARGO_OPTS) test

# TODO(jxson): derive the directory correctly.
PHONY: init
init: build # Run with: eval "$(make init)"
	@echo -e "export PATH=\"target/debug:\$${PATH}\""
	@echo "$$(cargo run -q -- --init)"
