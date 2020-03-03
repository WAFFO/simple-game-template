#!/bin/bash

cd "$(dirname $0)" || exit 1

cargo update;
cargo install -f wasm-bindgen-cli;
