# Creates "build/tags/<tag name>/<architecture>" folders containing the compiled Rust program as it was at that git tag.

set -euo pipefail

cd "${0%/*}/.."

tag=$1

# https://stackoverflow.com/questions/59838/how-do-i-check-if-a-directory-exists-or-not-in-a-bash-shell-script/59839#59839
if [ ! -d "build/tags/$tag" ]; then
  echo Building version $tag...
  
  # https://stackoverflow.com/questions/22861580/bash-script-check-if-a-file-contains-a-specific-line/69022922#69022922
  if
    [ -f "./scripts/lib/tag-ignore-list.txt" ] &&
    grep -Fxq "$tag" "./scripts/lib/tag-ignore-list.txt"
  then
    echo Error: tag $tag is in ignore list
    exit 1
  fi
  
  # https://unix.stackexchange.com/questions/786103/in-bash-how-to-capture-stdout-and-the-exit-code-of-a-command-when-the-e-flag-i/786104#786104
  # https://stackoverflow.com/questions/15541321/set-a-parent-shells-variable-from-a-subshell/15543655#15543655
  { nix --extra-experimental-features 'nix-command flakes' build git+file:.?ref=$tag#build-all-platforms --no-link && build_result=$?; } || build_result=$?
  
  if [ $build_result -ne 0 ]; then
    echo Error building version $tag
    exit $build_result
  fi
  
  store_path=$(nix --extra-experimental-features 'nix-command flakes' build git+file:.?ref=$tag#build-all-platforms --no-link --print-out-paths)
  
  cp -r $store_path build/tags/$tag
fi
