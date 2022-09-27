# WarpGUI
//TODO: Test & build status badges


Warp GUI is a basic implementation of the Warp feature set. It aims to pair nicely with Uplink, Satellite's minimal mobile application.

## Prerequisites

You'll need to install [`rust`](https://www.rust-lang.org/tools/install) and have `cmake`, `protobuf` and `make` installed. These are usually installed with a build tool package like `xcode-select --install` on mac. Or included in standard dev tooling on respective operating systems.

If you do not have protobuf installed you can install it using homebrew on Mac: `brew install protobuf`.

## Running

To run the app in dev mode simply run `cargo run`.

The executable has a few command line options that might come in handy:

- `--path` which changes the folder used for storage, currently it's .cache but later it will go somewhere else
- `--title` which comes in handy if you have more than one window open

From cargo you might run  `cargo run -- --title "User 1" --path .user1` and on some operating systems you can run with different options in two separate terminals
but you can also run something like this (depending on OS) on that second terminal `.\target\debug\warp_gui.exe --path .user2 --title "User 2"` (note you don't need the extra `--`).

## Local Testing

To spawn multiple instances of the app execute the test script `test/physical/launch_physical_test.sh`. This will spawn two named windows with local cache files in a `.temp` directory.

Please also read through the [Physical Testing Checklist](https://github.com/Satellite-im/WarpGUI/blob/dev/test/CHECKLIST.md) and ensure functionality before submitting a PR.

## Resetting Data

It's often necessary to reset your account for development, to do so just delete any `.temp` and `.cache` files within this project.
