
<h1 align="center">
  <a href="https://satellite.im" target="_blank">
  <img src="extra/images/logo.png" width=200 height=200/><br>
  Uplink
  </a>
</h1>

<h4 align="center">Privacy First, Modular, P2P messaging client built atop Warp.</h4>

<div align="center">
  <a href="https://github.com/satellite-im/Uplink/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/satellite-im/Uplink/actions/workflows/ci.yml/badge.svg" />
  </a>
  <a href="https://satellite.wiki" target="_blank">
      <img src="https://img.shields.io/static/v1?label=Docs&message=satellite.wiki&color=blue" alt="Uplink Docs">
  </a>
</div>
<br/>

Uplink is written in pure Rust with a UI in [Dioxus](https://github.com/DioxusLabs) (which is also written in Rust). It was developed to be a new foundation for the basic implementation of Warp features in a universal application.

The goal should be to build a hyper-customizable application that can run anywhere and support extensions.

## Features

// TODO

## Contributing

Guidelines for contributing to Lapce can be found in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Prerequisites

You'll need to install [`rust`](https://www.rust-lang.org/tools/install) and have `cmake`, `protobuf` and `make` installed. These are usually installed with a build tool package like `xcode-select --install` on mac. Or included in standard dev tooling on respective operating systems.

If you do not have protobuf installed you can install it using homebrew on Mac: `brew install protobuf`.

|Distribution|Commands|
|--|--|
|Debian & Ubuntu|apt get -y libgtk-3-dev  libwebkit2gtk-4.0-dev libappindicator3-dev protobuf-compiler|
|Arch|pacman -S gtk3 cmake protobuf|
|MacOS|brew install protobuf, xcode-select --install, curl https://sh.rustup.rs -sSf | sh -s -- -y|
|Windows|¯\\\_(ツ)\_/¯|

## Building from source

### Linux
// TODO
### Windows
¯\\\_(ツ)\_/¯

### Mac
// TODO

## Running

To run the app in dev mode simply run `cargo run`.

The executable has a few command line options that might come in handy:

- `--path` which changes the folder used for storage, currently it's .warp but later it will go somewhere else
- `--title` which comes in handy if you have more than one window open

### Extra Options

You can specify a window title and a custom path for storage which is useful for local AB testing.
`cargo run -- --title "User 1" --path .user1`
> Note: You can also pass these options directly to the binary by omitting the `--`.

## Local Testing

To spawn multiple instances of the app execute the test script `test/physical/launch_physical_test.sh`. This will spawn two named windows with local cache files in a `.temp` directory.

Please also read through the [Physical Testing Checklist](https://github.com/Satellite-im/Uplink/blob/dev/test/CHECKLIST.md) and ensure functionality before submitting a PR.

## Resetting Data

It's often necessary to reset your account for development, to do so just delete any `.temp` and `.warp` files within this project. Note that on most *nix based systems the .warp file is stored in your home directory unless otherwise configured.

## Contributions

All contributions are welcome! Please keep in mind we're still a relatively small team and any work done to make sure contributions don't cause bugs or issues in the application is much appreciated.

## Special Thanks

**Lapce** - For serving as a great open-source editor and helping me (matt) through so many headaches by providing your code to the world.
