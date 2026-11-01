# Creates "build/current/<tag name>/<architecture>" folders containing the compiled Rust program.

set -euo pipefail

cd "${0%/*}/.."

nix --extra-experimental-features 'nix-command flakes' run . -- "$@"
