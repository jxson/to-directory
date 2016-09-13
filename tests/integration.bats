#!/usr/bin/env bats

# TODO: setup path normalization like https://git.io/viE87

setup() {
  DIRECTORY=$(pwd)
  TARGET="${BATS_TEST_DIRNAME}/../target/debug/to-directory"
}

teardown() {
  pushd $DIRECTORY
}

@test "to-directory --init" {
  run ${TARGET} --init

  [ "$status" -eq 0 ]
}

@test "eval \$(to-directory --init)" {
  run ${TARGET} --init
  [ "$status" -eq 0 ]

  eval "${output}"
  output=$(type to)
  [[ "${output}" =~ "function" ]]
}

# * Both verbose flags work
# * Any other flags don't cd
# * Errors propagate
# * changes directory
