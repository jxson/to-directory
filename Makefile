MAKEFLAGS += --warn-undefined-variables
SHELL := /bin/bash
PATH := "deps/bin:${PATH}"

.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:
.SUFFIXES:

CARGO = cargo
CARGO_OPTS =

PHONY: all
all: build

PHONY: build
build:
	$(CARGO) $(CARGO_OPTS) build

PHONY: install
install:
	$(CARGO) $(CARGO_OPTS) build --release

PHONY: test
test:
	$(CARGO) $(CARGO_OPTS) test


PHONY: test-integration
test-integration: build
	@bats "tests/integration.bats"

# TODO(jxson): derive the directory correctly.
# http://stackoverflow.com/questions/18136918/how-to-get-current-relative-directory-of-your-makefile
PHONY: init
init: build # Run with: eval "$(make init)"
	@echo -e "export PATH=\"$(shell pwd)/target/debug:\$${PATH}\""
	@echo "$$(cargo run -q -- --init)"
