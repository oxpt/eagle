#! /usr/bin/env bash

# fail fast
set -eo pipefail
shopt -s inherit_errexit

cd crates/apps/xeejp
cargo install -q wasm-pack
wasm-pack $@
