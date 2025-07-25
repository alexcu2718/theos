#!/usr/bin/env bash


# ensure rustup is available
if ! command -v rustup &> /dev/null; then
  echo "rustup not installed. Please install rustup first."
  exit 1
fi

# enforce nightly installed (we need it )
if ! rustup show | grep -q 'nightly'; then
  echo "Rust nightly not found. Installing..."
  rustup install nightly
fi

# Ensure bootimage is installed (for nightly)
if ! cargo install --list | grep -q '^bootimage v'; then
  echo "'bootimage' not found. Installing..."
  cargo install bootimage
fi

# Build using nightly via rustup
echo "Building kernel"
if ! rustup run nightly cargo bootimage; then
    echo "Error: Kernel build failed."
    exit 1
fi

# Check QEMU is installed
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "qemu-system-x86_64 is not installed. Please install QEMU first."
    exit 1
fi

# Run the image
 qemu-system-x86_64 -drive format=raw,file=target/x86_64-nostd/debug/bootimage-theos.bin