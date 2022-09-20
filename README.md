# WarpGUI

# Prerequisites

You'll need to install `rust` and have `cmake`, `protobuf` and `make` installed. These are usually installed with a build tool package like `xcode-select --install` on mac. 

If you do not have protobuf installed you can install it using homebrew: `brew install protobuf`

# Running

To run the app in dev mode simply run `cargo run`

# Local Testing

To spawn multiple instances of the app execute the test script `test/physical/launch_physical_test.sh`. This will spawn two named windows with local cache files in a `.temp` directory.

# Resetting Data

It's often necessary to reset your account for development, to do so just delete any `.temp` and `.cache` files within this project.
