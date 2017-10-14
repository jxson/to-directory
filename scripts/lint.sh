set -e

source "$(git rev-parse --show-toplevel)/scripts/common.sh"

function main() {
  if [[ $TRAVIS_RUST_VERSION -eq "nightly" && -z $(which rustfmt) ]]; then
    echo "==> installing rustfmt-nightly"
    cargo install rustfmt-nightly
  fi

  echo "==> checking formatting"
  cargo fmt -- --write-mode=diff

  if [[ -z $(which cargo-clippy) ]]; then
    echo "==> installing cargo-clippy"
    cargo install clippy
  fi

  echo "==> checking clippy lints"
  cargo clippy
}

main "$@"
