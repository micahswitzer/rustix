#!/usr/bin/env bash
# install rust components
RUSTUP_COMPONENTS=$(rustup component list --installed)
[[ $RUSTUP_COMPONENTS != *rust-src* ]] &&
    rustup component add rust-src
[[ $RUSTUP_COMPONENTS != *llvm-tools-preview* ]] &&
    rustup component add llvm-tools-preview
# install rust binaries
RUST_BINARIES=$(cargo install --list)
[[ $RUST_BINARIES != *cargo-xbuild* ]] &&
    cargo install cargo-xbuild
[[ $RUST_BINARIES != *bootimage* ]] &&
    cargo install bootimage
