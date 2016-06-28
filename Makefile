MAKEFLAGS += --warn-undefined-variables
PATH := node_modules/.bin:$(PATH)
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
