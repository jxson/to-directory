set -e

source "$(git rev-parse --show-toplevel)/scripts/common.sh"

function main() {
  if [[ $TRAVIS_RUST_VERSION -eq "nightly" ]]; then
    echo "==> installing rustfmt-nightly"
    cargo install rustfmt-nightly --force

    echo "==> installing cargo-clippy"
    cargo install clippy --force

    echo "==> checking formatting"
    cargo fmt -- --write-mode=diff

    echo "==> checking clippy lints"
    cargo clippy
  fi
}

main "$@"
