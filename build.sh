#!/usr/bin/env bash

# Set required environment variables
export CARGO_MANIFEST_DIR=$(pwd)

# enforce  if nightly installed
if ! rustup show | grep -q 'nightly'; then
  echo "Rust nightly not found. Installing..."
  rustup install nightly
fi

# enforce  bootimage is installed
if ! cargo install --list | grep -q '^bootimage v'; then
  echo "'bootimage' not found. Installing..."
  cargo install bootimage
fi


# Run the build using nightly (a dependency of doing nostd bootloader fun!)
echo "Building kernel"
if ! cargo +nightly bootimage; then
    echo "Error: Kernel build failed."
    exit 1
fi


#check if qemu is installed 

if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "qemu-system-x86_64 is not installed. Please install QEMU first."
    exit 1
fi


QT_LOGGING_RULES="*.debug=false" qemu-system-x86_64 -drive format=raw,file=target/x86_64-nostd/debug/bootimage-theos.bin
