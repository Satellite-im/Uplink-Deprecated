#!/bin/sh
# Build a debug bin
cargo build
# Create temporary locations for the binaries
mkdir -p ./.temp/jane
mkdir -p ./.temp/john
# Clone the binaries
cp target/debug/uplink ./.temp/jane
cp target/debug/uplink ./.temp/john
# Run the binaries
cd ./.temp/jane && ./uplink --path jane_cache --title="Uplink - test[Jane Doe]" &
cd ./.temp/john && ./uplink --path john_cache --title="Uplink - test[John Doe]" &
