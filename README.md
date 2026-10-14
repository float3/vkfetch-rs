# vkfetch-rs

[![CI](https://github.com/float3/vkfetch-rs/actions/workflows/CI.yaml/badge.svg)](https://github.com/float3/vkfetch-rs/actions/workflows/CI.yaml)

`vkfetch-rs` prints Vulkan GPU information with vendor ASCII art. it is a Rust rewrite of [Wunkolo/vkfetch](https://github.com/Wunkolo/vkfetch), with the original art ported and a few extra device capability fields.

## install

you need a Vulkan loader at runtime.

```sh
cargo install vkfetch-rs
```

## build

```sh
git clone https://github.com/float3/vkfetch-rs
cd vkfetch-rs
cargo build
```

the default build links against the Vulkan loader. use the dynamic-loading feature when you want the binary to load Vulkan at runtime instead:

```sh
cargo build --no-default-features --features loaded
```

Nix users can enter a shell with the Vulkan loader, linker, rustup, and cargo-edit available:

```sh
nix develop
```

## development

the repository pins stable Rust with `rustfmt` and `clippy` in `rust-toolchain.toml`.

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo check --workspace --all-targets --no-default-features --features loaded
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
nix flake check --print-build-logs
```

useful maintenance commands:

```sh
cargo clippy --fix --workspace --all-targets --all-features --allow-dirty --allow-staged -- -D warnings
cargo upgrade -i
cargo update
```

## release

bump `package.version` in `Cargo.toml` and keep `Cargo.lock` in sync with `cargo update`. after that bump lands on `master` or `main`, CI runs the Rust and Nix checks, creates tag `vX.Y.Z`, builds release executables with the dynamic-loading Vulkan feature, and publishes a GitHub release with generated release notes plus archives for Linux, Windows, and macOS.
