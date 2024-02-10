#!/usr/bin/env bash
set -e
set -u

wasm-pack build --target web --out-dir ../target/www octant-client
cargo build -p octant-scoreboard
mkdir -p target/www
cp -f octant-client/www/index.html target/www/index.html
