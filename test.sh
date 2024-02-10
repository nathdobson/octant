#!/usr/bin/env bash
set -e
set -u
./build.sh
RUST_LOG=info cargo run -p octant-scoreboard -- --bind-http 0.0.0.0:8080
