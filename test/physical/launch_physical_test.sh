#!/bin/sh

if ! [ -x "$(command -v gum)" ]; then
  echo 'Gum not installed, spawning two instances.' >&2
  COUNT=2
else
  COUNT=$(gum input --placeholder "instance count")
fi

# Build a debug bin
cargo build

for ((n=0;n<$COUNT;n++))
do
    # Create temporary locations for the binaries
    mkdir -p ./.temp/user$n
    # Run the binaries
    target/debug/uplink --path ./.temp/user$n &
done