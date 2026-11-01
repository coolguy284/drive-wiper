# Enters a development environment where rustc and cargo have the
# same version that they do in the built 64-bit linux executable

set -euo pipefail

cd "${0%/*}/.."

nix --extra-experimental-features 'nix-command flakes' develop
