## Getting Started

Make sure you've got all the [prerequisites](#Prerequisites) setup in your local environment.

## Prerequisites

### 0. Build Tools, Dependencies

1. Install native build tools & debugger
   - On Windows, install Visual Studio 2022 with the C++ tools and Windows 10 SDK options.
   - On macOS, install the latest version of XCode with the relevant C++ build tools.
   - On linux, install the latest C++ dev tools via your package manager. There are rust toolchains for both gnu and musl, so pick your poison.
2. Install libpq
   - On Windows
     1. Install [vcpkg](https://vcpkg.io/en/getting-started.html)
     2. `./vcpkg.exe install libpq:x64-windows`
     3. `./vcpkg.exe integrate install`
   - On macos, `brew install libpq`
   - On linux, install libpq using your package manager. `diesel-cli` will link to libpq using your build tools, so make sure that the ABI is compatible with the toolchain ABI you chose.
3. Install [podman](https://podman.io/)
   - some of our toolchain expects the docker CLI. the podman CLI is compatbile with the docker CLI, so you can set an alias to podman. i.e `alias docker=podman`

### 1. Rust Toolchain

1. Install [rustup](https://github.com/rust-lang/rustup).
   - Read the [rustup book](https://rust-lang.github.io/rustup/index.html) in its entirety! It is important to be very familiar with the tools in our local environment we use for development; also, the book is very short and basically all the information is useful to us.
2. Install stable toolchain
   - `rustup toolchain install stable`
   - read more about toolchains [here](https://rust-lang.github.io/rustup/concepts/toolchains.html#toolchain-specification)
   - you can list every target triple with `rustup target list`

### 2. Project Tools

1. Install diesel-cli:
   - On windows: `VCPKGRS_DYNAMIC=1 cargo install diesel_cli --no-default-features --features postgres`
   - On macos/linux: `cargo install diesel_cli --no-default-features --features postgres`
   - Make sure to read the [diesel docs](http://diesel.rs/), this is our primary ORM.
   - Read the [CLI's readme](https://github.com/diesel-rs/diesel/tree/master/diesel_cli)
   - Read the [all the diesel guides](https://diesel.rs/guides/) to get a good understanding of how to approach designing clean code with diesel.
     - in particular, read [Composing Applications with Diesel](https://diesel.rs/guides/composing-applications.html)