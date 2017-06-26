MAKEFLAGS += --warn-undefined-variables
SHELL := /bin/bash
PATH := "$(shell pwd)/target/debug:${PATH}"

.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:
.SUFFIXES:

CARGO = $(shell which cargo)
CARGO_OPTS =

bats := "$(shell pwd)/vendor/bats/bin/bats"
bats_files := "$(shell find tests/integration -name '*.bats' )"

PHONY: all
all: build

PHONY: build
build:
	@$(CARGO) $(CARGO_OPTS) build

PHONY: test
test: test-rust test-bats

PHONY: test-rust
test-rust:
	@echo "== running: cargo test"
	@$(CARGO) $(CARGO_OPTS) test

PHONY: test-bats
test-bats: vendor/bats
	@echo "== running: bats test/integration/*"
	@$(bats) $(bats_files)

vendor/bats:
	mkdir -p $(dir $@)
	@git clone https://github.com/sstephenson/bats.git $@
	@touch $@

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

PHONY: lint
lint:
	@$(CARGO) $(CARGO_OPTS) clippy
