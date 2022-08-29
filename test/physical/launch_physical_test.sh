#!/bin/sh
cd ../../
# Build a debug bin
cargo build
# Create temporary locations for the binaries
mkdir -p ./.temp/jane
mkdir -p ./.temp/john
# Clone the binaries
cp target/debug/warp_gui ./.temp/jane
cp target/debug/warp_gui ./.temp/john
# Run the binaries
cd ./.temp/jane && ./warp_gui --path jane_cache --title="Warp GUI - test[Jane Doe]" &
cd ./.temp/john && ./warp_gui --path john_cache --title="Warp GUI - test[John Doe]" &
