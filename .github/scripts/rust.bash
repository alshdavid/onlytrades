#!/bin/bash
set -e

export RUSTUP_HOME="${HOME}/.local/rust/rustup"
export CARGO_HOME="${HOME}/.local/rust/cargo"

echo "RUSTUP_HOME=${HOME}/.local/rust/rustup" >> $GITHUB_ENV
echo "CARGO_HOME=${HOME}/.local/rust/cargo" >> $GITHUB_ENV

export PATH="${HOME}/.local/rust/cargo/bin:$PATH"
echo "${HOME}/.local/rust/cargo/bin" >> $GITHUB_PATH

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --no-modify-path -y

which cargo

rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-unknown-linux-musl
