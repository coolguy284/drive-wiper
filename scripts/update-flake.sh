# Updates the flake to latest to hopefully get the latest rust compilation toolkit.

set -euo pipefail

cd "${0%/*}/.."

nix --extra-experimental-features 'nix-command flakes' flake update
