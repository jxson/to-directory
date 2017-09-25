source "$(git rev-parse --show-toplevel)/scripts/common.sh"

function main() {
  if [[ $TRAVIS_RUST_VERSION -eq "nightly" && -z $(which rustfmt) ]]
  then
    echo "==> installing rustfmt-nightly"
    cargo install rustfmt-nightly
  fi

  cargo fmt -- --write-mode=diff
}

main "$@"
