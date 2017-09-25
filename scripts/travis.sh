export workspace="$(git rev-parse --show-toplevel)"
source "${workspace}/scripts/common.sh"

function main() {
  cargo build
  cargo test
  scripts/lint.sh
  scripts/coverage.sh
}

main "$@"
