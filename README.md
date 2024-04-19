ATAC ⚔📩
===

[![Rust](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml/badge.svg)](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Release](https://img.shields.io/github/v/release/julien-cpsn/ATAC?link=https%3A%2F%2Fgithub.com%2FJulien-cpsn%2FATAC%2Freleases%2Flatest)

![Demo Animation](./demo.gif)

## Table Of Contents

- [Description](#description)
- [How to install](#how-to-install)
  * [Install with cargo](#install-with-cargo)
  * [Install with Homebrew](#install-with-homebrew)
  * [Install from AUR](#install-from-aur)
  * [Binary](#binary)
  * [Compile by yourself](#compile-by-yourself)
- [Features](#features)
  * [Current](#current)
  * [TODO v1.0.0](#todo-v100)
  * [TODO v2.0.0](#todo-v200)
- [Documentation](#documentation)
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
> First, make sure your rust version is at least 1.76

Simply use:
```shell
cargo install atac
```

<a href="https://archlinux.org/packages/extra/x86_64/atac/">
  <img src="https://repology.org/badge/version-for-repo/arch/atac.svg" alt="Arch package" align="right">
</a>


### Install from AUR

You can use your favorite [AUR helper](https://wiki.archlinux.org/title/AUR_helpers) to install. For example,

```bash
paru -S atac
```


### Install with Homebrew

Simply use:

```bash
brew tap julien-cpsn/atac
brew install atac
```

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
> First, make sure your rust version is at least 1.76

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

| Features                            | **ATAC**                                                 | Postman              | Insomnia             |
|-------------------------------------|----------------------------------------------------------|----------------------|----------------------|
| **Manage collections & requests**   | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| **HTTP Client**                     | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| Methods                             | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - GET                               | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - POST                              | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - PUT                               | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - PATCH                             | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - DELETE                            | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - HEAD                              | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - OPTIONS                           | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| Authentication                      | Partial                                                  | :white_check_mark:   | :white_check_mark:   |
| - Basic auth                        | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Bearer token                      | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - JWT, Digest, OAuth1-2, AWS        | :x: :soon:                                               | :white_check_mark:   | :white_check_mark:   |
| Headers                             | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| Body                                | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Multipart form                    | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - URL Encoded form                  | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - File                              | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Plain text                        | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - JSON, XML, HTML                   | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| Full response                       | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Status code                       | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Cookies                           | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Headers                           | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Duration                          | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| Scripting                           | :x: :soon:                                               | Partial              | :x:                  |
| - Pre-request script                | :x: :soon:                                               | :x:                  | :x:                  |
| - Post-request script               | :x: :soon:                                               | :white_check_mark:   | :x:                  |
| Asynchronous requests               | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| Per-request settings                | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Use proxy                         | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Allow redirects                   | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Store cookies                     | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| **WebSocket Client**                | :x: :soon:                                               | :white_check_mark:   | :white_check_mark:   |
| **GraphQL**                         | :x: (not planned)                                        | :white_check_mark:   | :white_check_mark:   |
| **Free**                            | :white_check_mark:                                       | Depends              | Depends              |
| **Lightweight, fast and efficient** | :white_check_mark:                                       | :x:                  | :x:                  |
| **Data storage**                    | Your own committable, readable and versioned files       | Tied to your account | Tied to your account |
| **Offline**                         | :white_check_mark:                                       | :x:                  | :x:                  |
| **Real-time collaboration**         | :x: (not planned)                                        | :white_check_mark:   | :white_check_mark:   |
| **Environment files and variables** | :white_check_mark: (committable, readable and versioned) | :white_check_mark:   | :white_check_mark:   |
| **View options**                    | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| **Global configuration file**       | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - HTTP/HTTPS Proxy                  | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| - Disable CORS                      | :white_check_mark:                                       | :x:                  | :x:                  |
| - Toggle syntax highlighting        | :white_check_mark:                                       | :x:                  | :x:                  |
| Postman v2.1.0 import               | :white_check_mark:                                       | :white_check_mark:   | :white_check_mark:   |
| OpenAPI import                      | :x: :soon:                                               | :white_check_mark:   | :white_check_mark:   |

### TODO v1.0.0

- **To add**
  - Create a repo wiki
  - Document whole code

- **To improve**
  - Sign binary

- **To fix**
  - Query parameters bug

### TODO v2.0.0

- **To add**
  - Command line usage (send requests, add new requests)
  - keymap configuration (via a config file)
  - Request body syntax highlighting
  - Export a request to other code formats (curl, PHP, JS, Rust, ...)
  - Pre and post-request script (javascript v8 engine)

- **To improve**
  - Editing cookies
  - Insomnia import
  - Auto-completion on env file variables
  - Manage multipart Content-type header (auto-generated for now)

### Ideas (will think about it later)

- Base URL property on collections

## Documentation

> [!NOTE]
> Documentation will soon be published in the wiki section

## Technical precisions

### Tested on

- Console Host
  - Windows 11 (Pro)
  - WSL2 Debian
  - Windows 10 (Pro)
  - Windows 8.1 (N)
- Ubuntu Desktop Terminal
  - Ubuntu 17.10
  - Pop!_OS 20.04
- (Arch, Manjaro) KDE Konsole
- (Arch, NixOS) Kitty
- Linux Mint
- (OpenSuse) Alacritty
- (Chrome OS) Crostini

(List from [here](https://github.com/crossterm-rs/crossterm#tested-terminals))

### Dependencies

| Library                                                                                                                                    | Version           | Reason                                                                                 |
|--------------------------------------------------------------------------------------------------------------------------------------------|-------------------|----------------------------------------------------------------------------------------|
| [reqwest](https://github.com/seanmonstar/reqwest) & [reqwest cookie store](https://github.com/pfernie/reqwest_cookie_store)                | 0.11.27 & 0.6.0   | Send requests                                                                          |
| [ratatui](https://github.com/ratatui-org/ratatui)                                                                                          | 0.26.1            | Terminal UI framework                                                                  |
| [crossterm](https://github.com/crossterm-rs/crossterm)                                                                                     | 0.27.0            | Terminal Backend                                                                       |
| [tui-big-text](https://github.com/joshka/tui-big-text)                                                                                     | 0.4.2             | Display big texts. Only used for displaying ATAC in the homepage.                      |
| [tui-tree-widget](https://github.com/EdJoPaTo/tui-rs-tree-widget)                                                                          | 0.19.0            | Display tree-like lists. Used for displaying the collections.                          |
| [tui-textarea](https://github.com/rhysd/tui-textarea)                                                                                      | 0.5.0             | Text area that handle a lot of features. Used for editing request body.                |
| [throbber-widgets-tui](https://github.com/arkbig/throbber-widgets-tui)                                                                     | 0.4.1             | Display loading UI elements. Used when request is pending.                             |
| [syntect](https://github.com/trishume/syntect)                                                                                             | 5.2.0             | Syntax highlighting                                                                    |
| [serde](https://github.com/serde-rs/serde) & [serde_json](https://github.com/serde-rs/json)                                                | 1.0.197 & 1.0.144 | Serialize & Deserialize application data into JSON files                               |
| [jsonxf](https://github.com/gamache/jsonxf)                                                                                                | 0.1.1             | Pretty print JSON                                                                      |
| [toml](https://github.com/toml-rs/toml)                                                                                                    | 0.8.11            | Serialize & Deserialize application config files                                       |
| [envfile](https://github.com/pop-os/envfile)                                                                                               | 0.2.1             | Deserialize application environment files                                              |
| [My fork](https://github.com/Julien-cpsn/postman-collection-rs) of [postman_collection](https://github.com/mandrean/postman-collection-rs) | 0.2.1             | Deserialize Postman collection files                                                   |
| [clap](https://github.com/clap-rs/clap)                                                                                                    | 4.5.0             | Command Line Argument Parser                                                           |
| [tokio](https://github.com/tokio-rs/tokio)                                                                                                 | 1.0.0             | Handle asynchronous requests                                                           |
| [strum](https://github.com/Peternator7/strum)                                                                                              | 0.26.2            | Enum facilities                                                                        |
| [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs)                                                                         | 1.4.0             | Allows for more flexible constants. Mainly used for accessing CLI arguments everywhere |
| [regex](https://github.com/rust-lang/regex)                                                                                                | 1.10.3            | Regex. Using for parsing requests URL                                                  |

### Binary size

The binary file size goes from ~4.5 MB to ~7 MB depending on the platform. I try to keep it as small as possible.

## Contributors

### Maintainers

- [@julien-cpsn](https://github.com/julien-cpsn)

### Packagers

- Cargo, Brew - [@julien-cpsn](https://github.com/julien-cpsn)
- Arch - [@orhun](https://github.com/orhun)

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
