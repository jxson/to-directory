#!/usr/bin/env bats

# https://github.com/sstephenson/bats
# https://github.com/jxson/to-directory/blob/138803bc86369016fb237064032dfcda80aba11f/tests/integration.bats

load setup

@test "to --help" {
  run to --help

  [ $status -eq 0 ]
}
