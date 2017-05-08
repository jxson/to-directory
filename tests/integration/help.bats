#!/usr/bin/env bats

# https://github.com/jxson/to-directory/blob/138803bc86369016fb237064032dfcda80aba11f/tests/integration.bats

setup() {
  DIRECTORY=$(pwd)
  TARGET="${BATS_TEST_DIRNAME}/../../target/release/to-directory"
  cargo build --release
}

teardown() {
  pushd $DIRECTORY
}

@test "to --help" {
  run ${TARGET} --help

  [ $status -eq 0 ]
}
