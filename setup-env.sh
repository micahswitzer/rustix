#!/usr/bin/env bash
# install rust components
RUSTUP_COMPONENTS=$(rustup component list --installed)
if [[ $RUSTUP_COMPONENTS != *rust-src* ]]; then
	rustup component add rust-src
fi;
if [[ $RUSTUP_COMPONENTS != *llvm-tools-preview* ]]; then
	rustup component add llvm-tools-preview
fi;
# install rust binaries
RUST_BINARIES=$(cargo install --list)
if [[ $RUST_BINARIES != *cargo-xbuild* ]]; then
	cargo install cargo-xbuild
fi;
if [[ $RUST_BINARIES != *bootimage* ]]; then
	cargo install bootimage
fi;
