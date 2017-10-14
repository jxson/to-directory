set -e

source "$(git rev-parse --show-toplevel)/scripts/common.sh"

function main() {
  if [[ $TRAVIS_RUST_VERSION == "" || $TRAVIS_RUST_VERSION -ne "nightly" ]]
  then
    echo "==> coverage reports are only built on travis-ci and rustc 'nightly'"
    exit 0
  fi

  if [[ -z $(which kcov) ]]
  then
    echo "==> install kcov"
    cargo install cargo-kcov
    local tmpfile=$(mktemp)
    cargo kcov --print-install-kcov-sh >> "$tmpfile"
    sh "$tmpfile"
    rm "$tmpfile"
  fi

  if [[ $TRAVIS_RUST_VERSION -eq "nightly" ]]
  then
    echo "==> building coverage reports"
    cargo kcov --coveralls
  fi
}

main "$@"
