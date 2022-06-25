#!/bin/bash

spliter () { echo =================================================================; }

# spliter;
# cargo test --release

# spliter;
# cargo doc --release --open

spliter;
cargo fmt --all
# cargo build --release
cargo run --release -- $*

spliter;
# rm -r build/*
