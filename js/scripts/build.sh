#!/usr/bin/env bash

options=$(getopt -o 'ho:' --long 'help,mode:,root:' -n "$0" -- "$@")
if [[ $? -ne 0 ]]; then
    exit 1;
fi
eval set -- "$options"
unset options

help=$(
cat <<EOF
Usage: $0 [--mode TYPE] target

Build the js/wasm artifact.

Arguments
  TARGET    The target / bundler. Any bundler supported by "wasm-pack" is allowed here.

Options
  --mode    The optimization step. Can be either "dev" or "release". Defaults to "release".
  --root    The output directory. Defaults to "pkg".
EOF
);

root="pkg"
mode="release"
while true; do
    case "$1" in
        -h|--help)
            echo "$help"
            exit 0
            ;;

        --mode)
            mode="$2"
            shift 2
            ;;

        -o|--root)
            root="$2"
            shift 2
            ;;

        --)
            shift
            break
            ;;
    esac
done

if [[ "$#" -ne 1 ]]; then
    echo "missing target"
    echo
    echo "$help"
    exit 1
fi
target="$1"

[ -d pkg ] && rm -rf pkg

opts=("--out-name" "index" "--target" "$target" "--out-dir" "$root")
if [[ "$mode" == "dev" ]]; then
   opts+=("--dev")
fi

wasm-pack build "${opts[@]}"

jq '.name = "healpix-geo" | .repository.url = "git+https://github.com/GRID4EARTH/healpix-geo.git" | .["sideEffects"] = []' \
    pkg/package.json \
    > pkg/package.json.new
mv pkg/package.json.new pkg/package.json
