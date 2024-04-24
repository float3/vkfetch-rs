#!/usr/bin/env nix-shell
#!nix-shell -i bash -p cargo rustup openimagedenoise

cargo update --workspace 
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --workspace  -- -D warnings
cargo fix --allow-dirty --allow-staged --all-targets --all-features --workspace 
cargo fmt --all 
cargo check --all-targets --all-features --workspace --release
cargo test --all-targets --all-features --workspace --release 
cargo build --all-targets --all-features --workspace --release
cargo run --release