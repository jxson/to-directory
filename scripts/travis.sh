set -e

export workspace="$(git rev-parse --show-toplevel)"
source "${workspace}/scripts/common.sh"

function main() {
  RUSTFLAGS="-C link-dead-code" cargo test
  scripts/coverage.sh
}

main "$@"
