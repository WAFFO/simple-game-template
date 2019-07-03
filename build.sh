#!/bin/sh

set -ex
cd "$(dirname $0)"

rustup target add wasm32-unknown-unknown --toolchain stable

if ( ! command -v wasm-bindgen )
then
    cargo install wasm-bindgen-cli
fi

cargo build -p game --target wasm32-unknown-unknown
cargo build -p server

wasm-bindgen ./target/wasm32-unknown-unknown/debug/game.wasm --out-dir ./www/wasm

cp ./target/debug/server.exe ./server/build