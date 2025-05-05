ATAC ⚔📩
===

[![Rust](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml/badge.svg)](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Release](https://img.shields.io/github/v/release/julien-cpsn/ATAC?link=https%3A%2F%2Fgithub.com%2FJulien-cpsn%2FATAC%2Freleases%2Flatest)

![Demo Animation](gifs/demo.gif)

## Table Of Contents

- [Description](#description)
- [How to install](#how-to-install)
  * [Install with cargo](#install-with-cargo)
  * [Install with Homebrew](#install-with-homebrew)
  * [Install with Scoop](#install-with-scoop)
  * [Install from Arch](#install-from-arch)
  * [Install from Fedora copr](#install-from-fedora-copr)
  * [Docker package](#docker-package)
  * [Binary](#binary)
  * [Compile by yourself](#compile-by-yourself)
- [Features](#features)
  * [Current](#current)
  * [TODO v1.0.0](#todo-v100)
  * [TODO v2.0.0](#todo-v200)
- [Documentation](#documentation)
- [Others](#others)
  * [Vim key-bindings](#vim-key-bindings)
  * [NeoVim integration](#neovim-integration)
  * [Themes](#themes)
- [Technical precisions](#technical-precisions)
  * [Tested on](#tested-on)
  * [Dependencies](#dependencies)
  * [Binary size](#binary-size)
- [Contributors](#contributors)
  * [Maintainers](#maintainers)
  * [Packagers](#packagers)
- [Star history](#star-history)
- [License](#license)

## Description

ATAC is **A**rguably a **T**erminal **A**PI **C**lient.
It is based on well-known clients such as Postman, Insomnia, or even Bruno,
but inside your terminal without any specific graphical environment needed.

The philosophy of ATAC is to be free, account-less, and offline for now and forever.

## How to install

[![Packaging status](https://repology.org/badge/vertical-allrepos/atac.svg)](https://repology.org/project/atac/versions)

<a href="https://crates.io/crates/atac">
  <img src="https://repology.org/badge/version-for-repo/crates_io/atac.svg" alt="crates.io package" align="right">
</a>

### Install with cargo

> [!IMPORTANT]
> First, make sure your rust version is at least 1.79

Simply use:

```shell
cargo install atac --locked
```

<a href="https://archlinux.org/packages/extra/x86_64/atac/">
  <img src="https://repology.org/badge/version-for-repo/arch/atac.svg" alt="Arch package" align="right">
</a>

### Install from Arch

You can use [pacman](https://wiki.archlinux.org/title/pacman) to install:

```shell
pacman -S atac
```

### Install with Homebrew

Simply use:

```bash
brew tap julien-cpsn/atac
brew install atac
```

<a href="https://github.com/ScoopInstaller/Main/blob/master/bucket/atac.json">
  <img src="https://repology.org/badge/version-for-repo/scoop/atac.svg" alt="Scoop package" align="right">
</a>

### Install with Scoop

Simply use:

```bash
scoop install atac
```

<a href="https://copr.fedorainfracloud.org/coprs/joxcat/atac/">
  <img alt="Fedora copr Release" src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fcopr.fedorainfracloud.org%2Fapi_3%2Fpackage%3Fownername%3Djoxcat%26projectname%3Datac%26packagename%3Datac%26with_latest_build%3DTrue&query=%24.builds.latest.source_package.version&style=flat&logo=fedora&logoColor=white&label=Fedora%20copr&color=limegreen" align="right">
</a>

### Install from Fedora copr

Simply use:

```bash
dnf copr enable joxcat/atac
dnf install atac
```

<a href="https://hub.docker.com/repository/docker/juliencaposiena/atac">
 <img alt="Docker Image Version" src="https://img.shields.io/docker/v/juliencaposiena/atac?logo=docker" align="right">
</a>

### Docker package

Pull the image from https://hub.docker.com/repository/docker/juliencaposiena/atac/general

<a href="https://github.com/Julien-cpsn/ATAC/releases">
  <img alt="GitHub Release" src="https://img.shields.io/github/v/release/julien-cpsn/atac?label=Release&color=45c017" align="right">
</a>

### Binary

The binaries from the latest release can be found [here](https://github.com/Julien-cpsn/ATAC/releases/latest)

> [!IMPORTANT]
> Remember to run it from a terminal.
> For example, you can add the binary into your PATH.
> You won't be able to run it like other graphical applications since it needs CLI arguments.

> [!TIP]
> Note for **macOS users**. After downloading the binary you may need to run the command
> `sudo xattr -rd com.apple.quarantine ~/bin/atac` (modify to reflect the path where `atac` is located).

### Compile by yourself

> [!IMPORTANT]
> First, make sure your rust version is at least 1.79

Simply clone the repository and use:

```bash
cargo run
cargo run -- -h
```

> [!TIP]
> Build the latest release
> ```bash
> cargo build --release
> ```

## Features

### Current

| Features                            | **ATAC**                                                          | Postman              | Insomnia             |
|-------------------------------------|-------------------------------------------------------------------|----------------------|----------------------|
| **Manage collections & requests**   | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| **HTTP Client**                     | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| Methods                             | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - GET                               | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - POST                              | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - PUT                               | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - PATCH                             | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - DELETE                            | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - HEAD                              | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - OPTIONS                           | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| Authentication                      | Partial                                                           | :white_check_mark:   | :white_check_mark:   |
| - Basic auth                        | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Bearer token                      | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - JWT, Digest, OAuth1-2, AWS        | :x: :soon:                                                        | :white_check_mark:   | :white_check_mark:   |
| Headers                             | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| Body                                | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Multipart form                    | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - URL Encoded form                  | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - File                              | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Plain text                        | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - JSON, XML, HTML, Javascript       | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| Full response                       | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Status code                       | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Cookies                           | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Headers                           | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Duration                          | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| Scripting                           | :white_check_mark:                                                | Partial              | :x:                  |
| - Pre-request script                | :white_check_mark:                                                | :x:                  | :x:                  |
| - Post-request script               | :white_check_mark:                                                | :white_check_mark:   | :x:                  |
| Asynchronous requests               | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| Per-request settings                | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Use proxy                         | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Allow redirects                   | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Store cookies                     | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| Export to other languages           | :white_check_mark:                                                | :white_check_mark:   | :x:                  |
| **WebSocket Client**                | :x: :soon:                                                        | :white_check_mark:   | :white_check_mark:   |
| **GraphQL**                         | :x: :soon:                                                        | :white_check_mark:   | :white_check_mark:   |
| **gRPC**                            | :x: :soon:                                                        | :white_check_mark:   | :white_check_mark:   |
| **MQTT**                            | :x: :soon:                                                        | :white_check_mark:   | :x:                  |
| **Free**                            | :white_check_mark:                                                | Depends              | Depends              |
| **Lightweight, fast and efficient** | :white_check_mark:                                                | :x:                  | :x:                  |
| **Data storage**                    | Your own committable, readable and versioned files (JSON or YAML) | Tied to your account | Tied to your account |
| **Offline**                         | :white_check_mark:                                                | :x:                  | :x:                  |
| **Real-time collaboration**         | :x: (not planned)                                                 | :white_check_mark:   | :white_check_mark:   |
| **Full command line usage**         | :white_check_mark:                                                | Partial              | :x:                  |
| **Environment files and variables** | :white_check_mark: (committable, readable and versioned)          | :white_check_mark:   | :white_check_mark:   |
| **View options**                    | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| **Global configuration file**       | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - HTTP/HTTPS Proxy                  | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| - Disable CORS                      | :white_check_mark:                                                | :x:                  | :x:                  |
| - Toggle syntax highlighting        | :white_check_mark:                                                | :x:                  | :x:                  |
| Postman v2.1.0 import               | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |
| OpenAPI import                      | :white_check_mark: (AI generated, prone to bugs)                  | :white_check_mark:   | :white_check_mark:   |
| **Themes**                          | :white_check_mark:                                                | :white_check_mark:   | :white_check_mark:   |

### TODO v1.0.0

- **To add**
  - Individual request documentation in the TUI (markdown)
  - Log viewer in the TUI (likely with [tui-logger](https://github.com/gin66/tui-logger))
  - Env variables editor in the TUI
  - Handle path params
  - Insomnia import

- **To improve**
  - Editing cookies
  - Request body syntax highlighting
  - Manage multipart Content-type header (auto-generated for now) https://github.com/seanmonstar/reqwest/issues/2259

- **To fix**
  - Query parameters bug

### TODO v2.0.0

- **To add**
  - WebSocket requests
  - Maybe GraphQL requests
  - Maybe MQTT requests
  - Maybe gRPC requests

- **To improve**
  - Auto-completion on env file variables

### Ideas (will think about it later)

- Base URL property and authorization on collections
- VScode plugin to see and send requests

## Documentation

Here is the documentation: https://atac.julien-cpsn.com/

If you think something is missing, please contribute!

## Others

### Vim key-bindings

You can read more about it here: https://github.com/Julien-cpsn/ATAC/releases/tag/v0.14.0

### NeoVim integration

Thanks to [@NachoNievaG](https://github.com/NachoNievaG) you can have an ATAC floating window inside your nvim
https://github.com/NachoNievaG/atac.nvim

### Themes

You can read more about it here: https://github.com/Julien-cpsn/ATAC/releases/tag/v0.18.0

## Technical precisions

### Tested on

- Console Host
  - Windows 11 (Pro)
  - WSL2 Debian 12
  - Windows 10 (Pro)
  - Windows 8.1 (N)
- Ubuntu Desktop Terminal
  - Ubuntu 23.04 64-bit
  - Ubuntu 17.10
  - Pop!_OS 20.04
- (Arch, Manjaro) KDE Konsole
- (Arch, NixOS) Kitty
- Linux Mint
- (OpenSuse) Alacritty
- (Chrome OS) Crostini
- Apple
  - macOS Monterey 12.7.1 (Intel-Chip)
  - macOS Sonama 14.4 (M1 Max, Apple Silicon-Chip)

(List from [here](https://github.com/crossterm-rs/crossterm#tested-terminals))

### Dependencies

| Category / Library                                                                                                                               | Version                   | Reason                                                                                 |
|--------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------|----------------------------------------------------------------------------------------|
| **Request**                                                                                                                                      |                           |                                                                                        |
| [reqwest](https://github.com/seanmonstar/reqwest) & [reqwest cookie store](https://github.com/pfernie/reqwest_cookie_store)                      | 0.12.12 & 0.8.0           | Send requests                                                                          |
| **TUI**                                                                                                                                          |                           |                                                                                        |
| [ratatui](https://github.com/ratatui-org/ratatui)                                                                                                | 0.29.0                    | Terminal UI framework                                                                  |
| [crokey](https://github.com/Canop/crokey)                                                                                                        | 1.1.0                     | Used to parse, use key bindings files and some utilities                               |
| [tui-big-text](https://github.com/joshka/tui-big-text)                                                                                           | 0.7.1                     | Display big texts. Only used for displaying ATAC in the homepage.                      |
| [tui-tree-widget](https://github.com/EdJoPaTo/tui-rs-tree-widget)                                                                                | 0.23.0                    | Display tree-like lists. Used for displaying the collections.                          |
| [tui-textarea](https://github.com/rhysd/tui-textarea)                                                                                            | 0.7.0                     | Text area that handle a lot of features. Used for editing request body.                |
| [throbber-widgets-tui](https://github.com/arkbig/throbber-widgets-tui)                                                                           | 0.8.0                     | Display loading UI elements. Used when request is pending.                             |
| [ratatui-image](https://github.com/benjajaja/ratatui-image)                                                                                      | 4.2.0                     | Display response images.                                                               |
| [image](https://github.com/image-rs/image)                                                                                                       | 0.25.5                    | Decode images.                                                                         |
| **Main functionalities**                                                                                                                         |                           |                                                                                        |
| [syntect](https://github.com/trishume/syntect)                                                                                                   | 5.2.0                     | Syntax highlighting                                                                    |
| [serde](https://github.com/serde-rs/serde) ([serde_json](https://github.com/serde-rs/json), [serde-yaml](https://github.com/dtolnay/serde-yaml)) | 1.0.217 (1.0.139, 0.9.34) | Serialize & Deserialize application data into files                                    |
| [jsonxf](https://github.com/gamache/jsonxf)                                                                                                      | 1.1.1                     | Pretty print JSON                                                                      |
| [toml](https://github.com/toml-rs/toml)                                                                                                          | 0.8.20                    | Serialize & Deserialize application config files                                       |
| [boa_engine](https://github.com/boa-dev/boa)                                                                                                     | 0.20.0                    | Create Javascript runtimes. Used for pre and post request scripts                      |
| [My fork](https://github.com/Julien-cpsn/postman-collection-rs) of [postman_collection](https://github.com/mandrean/postman-collection-rs)       | 0.2.3                     | Deserialize Postman collection files                                                   |
| [curl-parser](https://github.com/tyrchen/curl-parser)                                                                                            | 0.5.0                     | Parse cURL request files                                                               |
| [openapiv3](https://github.com/glademiller/openapiv3)                                                                                            | 2.0.0                     | Parse OpenAPI spec files                                                               |
| [clap](https://github.com/clap-rs/clap)                                                                                                          | 4.5.30                    | Command Line Argument Parser                                                           |
| [directories](https://github.com/dirs-dev/directories-rs)                                                                                        | 6.0.0                     | Use system files                                                                       |
| [arboard](https://github.com/1Password/arboard)                                                                                                  | 3.4.1                     | Copy response body to clipboard                                                        |
| **Async**                                                                                                                                        |                           |                                                                                        |
| [tokio](https://github.com/tokio-rs/tokio)                                                                                                       | 1.43.0                    | Handle asynchronous requests                                                           |
| [parking_lot](https://github.com/Amanieu/parking_lot)                                                                                            | 0.12.3                    | Smaller, faster and more flexible implementation of RwLock and Mutex. Used everywhere. |
| **Utils**                                                                                                                                        |                           |                                                                                        |
| [strum](https://github.com/Peternator7/strum)                                                                                                    | 0.27.1                    | Enum facilities                                                                        |
| [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs)                                                                               | 1.5.0                     | Allows for more flexible constants. Mainly used for accessing CLI arguments everywhere |
| [nestify](https://github.com/snowfoxsh/nestify)                                                                                                  | 0.3.3                     | Used to nest struct definitions                                                        |
| [walkdir](https://github.com/BurntSushi/walkdir)                                                                                                 | 2.5.0                     | Recursively retrieve files                                                             |
| [snailquote](https://github.com/euank/snailquote)                                                                                                | 0.3.1                     | Unescape string                                                                        |
| [indexmap](https://github.com/indexmap-rs/indexmap)                                                                                              | 2.7.1                     | Ordered hashmap. Used in environments to preserve files' values order                  |
| [rayon](https://github.com/rayon-rs/rayon)                                                                                                       | 1.10.0                    | Allows the usage of multiple threads in for loops                                      |
| [thiserror](https://github.com/dtolnay/thiserror)                                                                                                | 2.0.11                    | Create custom errors                                                                   |
| [anyhow](https://github.com/dtolnay/anyhow)                                                                                                      | 1.0.96                    | Result that can contain any error                                                      |
| [clap-verbosity-flag](https://github.com/clap-rs/clap-verbosity-flag)                                                                            | 3.0.2                     | Add verbosity flag to the CLI                                                          |
| [clap_complete](https://github.com/clap-rs/clap/tree/master/clap_complete)                                                                       | 4.5.45                    | Generate completion file                                                               |
| [clap_mangen](https://github.com/clap-rs/clap/tree/master/clap_mangen)                                                                           | 0.2.26                    | Generate man pages                                                                     |
| [regex](https://github.com/rust-lang/regex)                                                                                                      | 1.11.1                    | Regex. Using for parsing requests URL                                                  |
| [chrono](https://github.com/chronotope/chrono)                                                                                                   | 0.4.39                    | Time utils                                                                             |
| [uuid](https://github.com/uuid-rs/uuid)                                                                                                          | 1.13.2                    | UUID generator                                                                         |
| **Tracing/Log**                                                                                                                                  |                           |                                                                                        |
| [tracing](https://github.com/tokio-rs/tracing)                                                                                                   | 0.1.41                    | Log events                                                                             |
| [tracing-subscriber](https://github.com/tokio-rs/tracing/tree/master/tracing-subscriber)                                                         | 0.3.19                    | Utilities for implementing and composing tracing subscribers                           |
| [tracing-log](https://github.com/tokio-rs/tracing/tree/master/tracing-log)                                                                       | 0.2.0                     | Log crate compatibility for tracing                                                    |
| [reqwest-tracing](https://github.com/TrueLayer/reqwest-middleware/tree/main/reqwest-tracing)                                                     | 0.5.5                     | Opentracing middleware implementation for reqwest-middleware                           |
| [reqwest-middleware](https://github.com/TrueLayer/reqwest-middleware)                                                                            | 0.4.0                     | Wrapper around reqwest to allow for client middleware chains                           |

### Binary size

The binary file size goes from ~5 MB to ~12 MB depending on the platform. I try to keep it as small as possible, unfortunately for me, I bundle a Javascript runtime.

## Contributors

### Maintainers

- [@julien-cpsn](https://github.com/julien-cpsn)

### Packagers

- Cargo, Brew, Docker - [@julien-cpsn](https://github.com/julien-cpsn)
- Arch - [@orhun](https://github.com/orhun)
- Fedora copr - [@joxcat](https://github.com/joxcat)

## Star history

<a href="https://star-history.com/#julien-cpsn/atac&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=julien-cpsn/atac&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=julien-cpsn/atac&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=julien-cpsn/atac&type=Date" />
 </picture>
</a>

## License

The MIT license for this project can be seen [here](./LICENSE)
