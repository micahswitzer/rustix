#!/usr/bin/env bash
rustup component add rust-src llvm-tools-preview
cargo install cargo-xbuild bootimage
