<h1 align="center">
  <a href="https://satellite.im" target="_blank">
  <img src="extra/images/logo.png" width=200 height=200/><br>
  WarpGUI
  </a>
</h1>

<h4 align="center">Privacy First, Modular, P2P messaging client built atop Warp.</h4>

<div align="center">
  <a href="https://github.com/satellite-im/WarpGUI/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/satellite-im/WarpGUI/actions/workflows/ci.yml/badge.svg" />
  </a>
  <a href="https://satellite.wiki" target="_blank">
      <img src="https://img.shields.io/static/v1?label=Docs&message=satellite.wiki&color=blue" alt="WarpGUI Docs">
  </a>
</div>
<br/>

Warp GUI is written in pure Rust with a UI in [Dioxus](https://github.com/DioxusLabs) (which is also written in Rust). It was developed to be a new foundation for basic implementation of Warp features in a universal application.

The goal should be to build a hyper customizable application that can run anywhere and support extensions.

## Features

* Built-in LSP ([Language Server Protocol](https://microsoft.github.io/language-server-protocol/)) support to give you intelligent code features such as: completion, diagnostics and code actions
* Modal editing support as first class citizen (Vim-like, and toggleable)
* Built-in remote development support inspired by [VSCode Remote Development](https://code.visualstudio.com/docs/remote/remote-overview). Enjoy the benefits of a "local" experience, and seamlessly gain the full power of a remote system.
* Plugins can be written in programming languages that can compile to the [WASI](https://wasi.dev/) format (C, Rust, [AssemblyScript](https://www.assemblyscript.org/))
* Built-in terminal, so you can execute commands in your workspace, without leaving Lapce.

## Contributing

Guidelines for contributing to Lapce can be found in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Installation

### Arch Linux

There is an community package that can be installed with `pacman`:

```bash
sudo pacman -Syu lapce
```

### Fedora
```bash
sudo dnf copr enable titaniumtown/lapce
sudo dnf install lapce
```

### Flatpak

Lapce is available as a flatpak [here](https://flathub.org/apps/details/dev.lapce.lapce)

```bash
flatpak install flathub dev.lapce.lapce
```

### Homebrew

```bash
brew install lapce
```

### Scoop

```bash
scoop install lapce
```

### winget

You can find the packages [here](https://github.com/microsoft/winget-pkgs/tree/master/manifests/l/Lapce/Lapce):

```bash
winget install lapce
```

## Building from source

It is easy to build Lapce from source on a GNU/Linux distribution. Cargo handles the build process, all you need to do, is ensure the correct dependencies are installed.

1. Install the Rust compiler and Cargo using [`rustup.rs`](https://rustup.rs/). If you already have the toolchain, ensure you are using version 1.62 or higher.

2. Install dependencies for your operating system:

#### Ubuntu
```sh
sudo apt install cmake pkg-config libfontconfig-dev libgtk-3-dev
```
#### Fedora
```sh
sudo dnf install gcc-c++ perl-FindBin perl-File-Compare gtk3-devel
```

3. Clone this repository (this command will clone to your home directory):
```sh
git clone https://github.com/lapce/lapce.git ~/lapce
```

4. `cd` into the repository, and run the build command with the release flag
```sh
cd ~/lapce
```

```sh
cargo build --release
```

> If you use a different distribution, and are having trouble finding appropriate dependencies, let us know in an issue!

Once Lapce is compiled, the executable will be available in `target/release/lapce`.

## Feedback & Contact

The most popular place for Lapce developers and users is on the [Discord server](https://discord.gg/n8tGJ6Rn6D).

Or, join the discussion on [Reddit](https://www.reddit.com/r/lapce/) where we are just getting started.

There is also a [Matrix Space](https://matrix.to/#/#lapce-editor:matrix.org), which is linked to the content from the Discord server.

## License

Lapce is released under the Apache License Version 2, which is an open source license. You may contribute to this project, or use the code as you please as long as you adhere to its conditions. You can find a copy of the license text here: [`LICENSE`](LICENSE).

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

- `--path` which changes the folder used for storage, currently it's .warp but later it will go somewhere else
- `--title` which comes in handy if you have more than one window open

### Extra Options

You can specify a window title and a custom path for storage which is useful for local AB testing.
`cargo run -- --title "User 1" --path .user1`
> Note: You can also pass these options directly to the binary by omitting the `--`.

## Local Testing

To spawn multiple instances of the app execute the test script `test/physical/launch_physical_test.sh`. This will spawn two named windows with local cache files in a `.temp` directory.

Please also read through the [Physical Testing Checklist](https://github.com/Satellite-im/WarpGUI/blob/dev/test/CHECKLIST.md) and ensure functionality before submitting a PR.

## Resetting Data

It's often necessary to reset your account for development, to do so just delete any `.temp` and `.warp` files within this project.

## Contributions

All contributions are welcome! Please keep in mind we're still a relatively small team and any work done to make sure contributions don't cause bugs or issues in the application are much appriciated.

## Special Thanks

**Lapce** - For serving as a great open source editor and helping me (matt) through so many headaches by providing your code to the world.