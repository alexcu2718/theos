#!/usr/bin/env bash
set -e

# check if nightly installed
if ! rustup show | grep -q 'nightly'; then
  echo "Rust nightly not found. Installing..."
  rustup install nightly
fi

# Ensure bootimage is installed
if ! cargo install --list | grep -q '^bootimage v'; then
  echo "'bootimage' not found. Installing..."
  cargo install bootimage
fi


# Run the build ussing nightly (a dependency of doing nostd bootloader fun!)
cargo +nightly build -r


cargo +nightly bootimage
