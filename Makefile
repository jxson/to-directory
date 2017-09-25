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
	scripts/lint.sh
	@$(CARGO) $(CARGO_OPTS) test

PHONY: test-bats
test-bats: vendor/bats
	@echo "==> running: bats test/integration/*"
	@$(bats) $(bats_files)

vendor/bats:
	mkdir -p $(dir $@)
	@git clone https://github.com/sstephenson/bats.git $@
	@touch $@

PHONY: install
install:
	cargo build --release
	cp target/release/to-directory "$$(brew --prefix)/bin"
