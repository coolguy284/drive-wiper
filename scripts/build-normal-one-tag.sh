# Creates "build/tags/<tag name>/<architecture>" folders containing the compiled Rust program as it was at that git tag.

# Usage:
#   <script file> <tag name> <path options>
# path options:
#   --no-link
#     nix build will not produce an output symlink
#   --local-link (default)
#     output symlink path is build/symlinks
#   --home-link
#     output symlink path is ~/.cache/git-nix-symlinks/<repo author>/<repo name>
#   --custom-path-link <path>
#     output symlink path is the path given
#   --custom-link <location>
#     output symlink path is the path given without the additional augmentation below
# if output link is to be created nix build will put output link at:
#   <path>/tags-normal/<tag name>

set -euo pipefail

orig_cwd=$(pwd)

cd "${0%/*}/.."

repo_root=$(pwd)

. "$repo_root/scripts/lib/symlink-args-gen.sh"

tag=$1
# https://www.google.com/search?q=bash+(%2524%7B*%253A1%7D)+splits+by+spaces
options=("${@:2}")

# https://stackoverflow.com/questions/59838/how-do-i-check-if-a-directory-exists-or-not-in-a-bash-shell-script/59839#59839
if [ ! -d "build/tags/$tag" ]; then
  echo Building version $tag...
  
  # https://stackoverflow.com/questions/22861580/bash-script-check-if-a-file-contains-a-specific-line/69022922#69022922
  if
    [ -f "./scripts/lib/tag-ignore-list.txt" ] &&
    grep -Fxq "$tag" "./scripts/lib/tag-ignore-list.txt"
  then
    echo Error: tag $tag is in ignore list 1>&2
    exit 1
  fi
  
  cd "$orig_cwd"
  
  # https://unix.stackexchange.com/questions/717945/prevent-splitting-command-line-argument-at-spaces-when-running-in-bash-script/717949#717949
  symlinkArgsGen "$orig_cwd" "$repo_root" normal tag $tag "${options[@]}"
  symlink_options=("${symlinkArgsGen_return[@]}")
  
  # https://unix.stackexchange.com/questions/786103/in-bash-how-to-capture-stdout-and-the-exit-code-of-a-command-when-the-e-flag-i/786104#786104
  # https://stackoverflow.com/questions/15541321/set-a-parent-shells-variable-from-a-subshell/15543655#15543655
  { { store_path=$(nix --extra-experimental-features 'nix-command flakes' build "git+file:$repo_root?ref=$tag#build-all-platforms" "${symlink_options[@]}" --print-out-paths); } && build_result=$?; } || build_result=$?
  
  if [ $build_result -ne 0 ]; then
    echo Error building version $tag
    exit $build_result
  fi
  
  cd "$repo_root"
  
  cp -r $store_path build/tags/$tag
fi
