#!/bin/sh

set -ex
cd "$(dirname $0)"

rustup target add wasm32-unknown-unknown --toolchain stable

if ( ! command -v wasm-bindgen )
then
    cargo install wasm-bindgen-cli
fi

cargo build --release -p game --target wasm32-unknown-unknown
cargo build --release -p server

wasm-bindgen ./target/wasm32-unknown-unknown/release/game.wasm --out-dir ./www/wasm
npm run --prefix ./www build || STATUS=$? && true ;

if [[ "$STATUS" -ne "0" ]]; then
    cd www
    npm install
    cd ..
    npm run --prefix ./www build
fi

cp ./target/release/server.exe ./server/build