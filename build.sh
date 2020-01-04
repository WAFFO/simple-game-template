#!/bin/sh

set -ex
cd "$(dirname $0)"

unameOut="$(uname -s)";
case "${unameOut}" in
    Linux*)     machine=Linux;;
    Darwin*)    machine=Mac;;
    CYGWIN*)    machine=Cygwin;;
    MINGW*)     machine=MinGw;;
    *)          machine="UNKNOWN:${unameOut}"
esac

rustup target add wasm32-unknown-unknown --toolchain stable

if ( ! command -v wasm-bindgen )
then
    cargo install wasm-bindgen-cli
fi

cargo build -p game --target wasm32-unknown-unknown
cargo build -p server

wasm-bindgen ./target/wasm32-unknown-unknown/debug/game.wasm --out-dir ./www/wasm

if [[ "${machine}" == "MinGw" ]] || [[ "${machine}" == "Cygwin" ]]; then
    cp ./target/debug/server.exe ./server/build
else
    cp ./target/debug/server ./server/build
fi;
