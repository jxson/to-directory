set -e

export workspace="$(git rev-parse --show-toplevel)"
source "${workspace}/scripts/common.sh"

function main() {
  cargo build
  cargo test
  scripts/coverage.sh
}

main "$@"
