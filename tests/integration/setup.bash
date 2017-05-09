SRC="${BATS_TEST_DIRNAME}/../../";
export PATH="${PATH}:${SRC}/target/debug";

setup() {
  DIRECTORY=$(pwd)
  if [[ ! -f to-directory ]]; then
    cargo build
  fi

  eval "$(to-directory --init)"
}

teardown() {
  pushd $DIRECTORY
}
