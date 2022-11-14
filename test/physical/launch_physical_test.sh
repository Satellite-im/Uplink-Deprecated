#!/bin/sh
COUNT=$(gum input --placeholder "instance count")
# Build a debug bin
cargo build

for ((n=0;n<$COUNT;n++))
do
    # Create temporary locations for the binaries
    mkdir -p ./.temp/user$n
    # Run the binaries
    target/debug/uplink --path ./.temp/user$n --title="Uplink - test[User $n]" &
done