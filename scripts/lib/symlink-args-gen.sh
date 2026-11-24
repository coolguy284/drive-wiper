# This script must be run in the same script instance (using . ./<script path>.sh), as an import
# This script requires the presence of a $repo_root variable with the path to the root of the git repo this is contained in

. "$repo_root/scripts/lib/constants.sh"

getSubPath(){
  # "normal" or "github"
  mode=$1
  # "current" or "tag"
  build_out_type=$2
  # "" for build_out_type current
  tag_name=$3
  
  if [ "$build_out_type" == "current" ]; then
    if [ "$mode" == "normal" ]; then
      echo "current-normal"
    elif [ "$mode" == "github" ]; then
      echo "current-github"
    else
      # https://stackoverflow.com/questions/30078281/raise-error-in-a-bash-script/30078353#30078353
      echo "mode not recognized: $mode" 1>&2
      exit 1
    fi
  elif [ "$build_out_type" == "tag" ]; then
    if [ "$mode" == "normal" ]; then
      echo "tags-normal/$tag_name"
    elif [ "$mode" == "github" ]; then
      echo "tags-github/$tag_name"
    else
      # https://stackoverflow.com/questions/30078281/raise-error-in-a-bash-script/30078353#30078353
      echo "mode not recognized: $mode" 1>&2
      exit 1
    fi
  else
    # https://stackoverflow.com/questions/30078281/raise-error-in-a-bash-script/30078353#30078353
    echo "build_out_type not recognized: $build_out_type" 1>&2
    exit 1
  fi
}

# https://stackoverflow.com/questions/56456794/how-to-return-array-from-bash-function/56456878#56456878
symlinkArgsGen(){
  orig_cwd=$1
  repo_root=$2
  # "normal" or "github"
  mode=$3
  # "current" or "tag"
  build_out_type=$4
  # "" for build_out_type current
  tag_name=$5
  # https://www.google.com/search?q=bash+(%2524%7B*%253A1%7D)+splits+by+spaces
  cli_args=("${@:6}")
  
  # https://stackoverflow.com/questions/7832080/test-if-a-variable-is-set-in-bash-when-using-set-o-nounset/7832158#7832158
  if [ -z "${cli_args:-}" ] || [ "${cli_args[0]}" == "--local-link" ]; then
    # local link (default)
    symlinkArgsGen_return=("--out-link" "$repo_root/build/symlinks/$(getSubPath $mode $build_out_type $tag_name)")
  elif [ "${cli_args[0]}" == "--home-link" ]; then
    # home link
    # https://superuser.com/questions/271925/where-is-the-home-environment-variable-set
    symlinkArgsGen_return=("--out-link" "$HOME/.cache/git-nix-symlinks/$REPO_PATH/$(getSubPath $mode $build_out_type $tag_name)")
  elif [ "${cli_args[0]}" == "--custom-path-link" ]; then
    # custom path link
    symlinkArgsGen_return=("--out-link" "${cli_args[1]}/$(getSubPath $mode $build_out_type $tag_name)")
  elif [ "${cli_args[0]}" == "--custom-link" ]; then
    # custom link
    symlinkArgsGen_return=("--out-link" "${cli_args[1]}")
  elif [ "${cli_args[0]}" == "--no-link" ]; then
    # no link
    symlinkArgsGen_return=("--no-link")
  else
    echo "cli arg 0 not recognized: ${cli_args[0]}" 1>&2
    exit 1
  fi
}
