#!/usr/bin/env bats

# TODO: setup path normalization like https://git.io/viE87

# setup() {

#     # Before each setup...
# }

setup() {
  TARGET="target/debug/to-directory"
  DIRECTORY=$(pwd)

  # Before all tests run make sure the rust binary is built.
  if [ "${BATS_TEST_NUMBER}" = "1" ]; then
    # Only build if the build target doesn't exist.
    if [ ! -f "${TARGET}" ]; then
      cargo build
    fi
  fi
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
