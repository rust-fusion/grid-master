#!/bin/bash

# Build the project for the web
cargo build --release --target wasm32-unknown-unknown

# Install wasm-bindgen CLI tool (if not already installed)
cargo install wasm-bindgen-cli --force

# Create the output directory
mkdir -p static

# Generate JavaScript bindings for the WebAssembly code
wasm-bindgen ./target/wasm32-unknown-unknown/release/my_game.wasm --out-dir static --no-typescript

