# WarpGUI
//TODO: Test & build status badges


Warp GUI is a basic implementation of the Warp feature set. It aims to pair nicely with Uplink, Satellite's minimal mobile application.

## Prerequisites

You'll need to install [`rust`](https://www.rust-lang.org/tools/install) and have `cmake`, `protobuf` and `make` installed. These are usually installed with a build tool package like `xcode-select --install` on mac. Or included in standard dev tooling on respective operating systems.

If you do not have protobuf installed you can install it using homebrew on Mac: `brew install protobuf`.

### Windows
// TODO:

### Mac

Firstly install [Homebrew](https://brew.sh/).

Next you'll want to make sure you have the build tools installed by running `xcode-select --install`.

Now go ahead and install `protobuf` using Brew:

```sh
brew install protobuf
```

## Running

To run the app in dev mode simply run `cargo run`.

The executable has a few command line options that might come in handy:

- `--path` which changes the folder used for storage, currently it's .cache but later it will go somewhere else
- `--title` which comes in handy if you have more than one window open

### Extra Options

You can specify a window title and a custom path for storage which is useful for local AB testing.
`cargo run -- --title "User 1" --path .user1`
> Note: You can also pass these options directly to the binary by omitting the `--`.

## Local Testing

To spawn multiple instances of the app execute the test script `test/physical/launch_physical_test.sh`. This will spawn two named windows with local cache files in a `.temp` directory.

Please also read through the [Physical Testing Checklist](https://github.com/Satellite-im/WarpGUI/blob/dev/test/CHECKLIST.md) and ensure functionality before submitting a PR.

## Resetting Data

It's often necessary to reset your account for development, to do so just delete any `.temp` and `.cache` files within this project.

## Contributions

All contributions are welcome! Please keep in mind we're still a relatively small team and any work done to make sure contributions don't cause bugs or issues in the application are much appriciated.
