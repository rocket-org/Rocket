#!/bin/bash -ex

# Usage:
#
#     ./update-libfuzzer $commit_hash
#
# Where `$commit_hash` is a commit hash from
# https://github.com/llvm-mirror/llvm-project

set -ex

cd "$(dirname $0)"
project_dir="$(pwd)"

tmp_dir="$(mktemp -d)"

git clone -b "$1" --single-branch https://github.com/llvm/llvm-project.git "$tmp_dir" \
&& mv "$project_dir/libfuzzer" "$project_dir/libfuzzer.$(date +%Y%m%d%H%M%S)" \
&& mv "$tmp_dir/compiler-rt/lib/fuzzer" "$project_dir/libfuzzer"
[ -d "$tmp_dir" ] && rm -rf "$tmp_dir"
