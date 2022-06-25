#!/bin/bash

spliter () { echo =================================================================; }

# spliter;
# cargo test --release

# spliter;
# cargo doc --release

spliter;
cargo fmt --all
# cargo build --release -q
cargo run --release -q -- $*

spliter;
# rm -r build/*
