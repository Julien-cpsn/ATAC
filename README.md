ATAC ⚔📩
===

[![Rust](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml/badge.svg)](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Release](https://img.shields.io/github/v/release/julien-cpsn/ATAC?link=https%3A%2F%2Fgithub.com%2FJulien-cpsn%2FATAC%2Freleases%2Flatest)

![Demo Animation](./demo.gif)

## Table Of Contents

- [Description](#description)
- [How to use](#how-to-use)
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
- [License](#license)

## Description

ATAC is **A**rguably a **T**erminal **A**PI **C**lient. It is based on well known clients such as Postman, Insomnia or even Bruno, but inside your terminal without any specific graphical environment needed.

The philosophy of ATAC is to be free, account-less and offline for now and forever.

## How to use

### Binary

The binaries from the latest release can be found [here](https://github.com/Julien-cpsn/ATAC/releases/latest)

> [!IMPORTANT]
> Don't forget to run it from a terminal. For example, you can add the binary into your PATH. You won't be able to run it like other graphical applications since it needs CLI arguments.

For more:

```
atac -h
```

### Compile by yourself

> [!IMPORTANT]
> First, make sure your rust version is at least 1.76

Simply clone the repository and use:

```bash
cargo run -- -h
```

> Build latest release 
> ```bash
> cargo build --release
> ```

## Features

### Current

- **Manage many collection**
- **Requests**
  - **Modify URL**
  - **All HTTP methods are handled**
    - GET
    - POST
    - PUT
    - PATCH
    - DELETE
    - HEAD
    - OPTIONS
  - **Query params**, synchronous between the URL and the query params tab
  - **Authentication**
    - No Auth
    - Basic Auth
    - Bearer Token
  - **Headers**
  - **Body**
    - No Body
    - Multipart form
    - URL encoded form
    - Plain Text
    - JSON
    - HTML
    - XML
  - **Full response**
    - Body (with automatic syntax highlighting)
    - Status code
    - Cookies
    - Request duration
  - **Simultaneously send asynchronous requests**
  - Settings
    - Use config proxy
    - Allow redirects
    - Store received cookies
- **Readable, commitable and versionable JSON files** containing application data
- Configuration file
  - **HTTP and HTTPS proxy**
  - Disable CORS
  - Disable syntax highlighting
- **Postman collection v2.1.0 import**
- **Environment variables** along .env file support
- Permanent keymap help & application state
- **3 request views**
  - 50% params / 50% response
  - 100% response
  - 100% params

### TODO v1.0.0

- **To add**
  - Cookies management
  - Insomnia import
  - Create a repo wiki
  - Document whole code

- **To improve**
  - Move requests and collections
  - Add Multipart form, URL encoded form and request settings from Postman import

### TODO v2.0.0

- **To add**
  - Command line usage (send requests, add new requests)
  - keymap configuration (via a config file)
  - Request body syntax highlighting
  - Export a request to other code formats (curl, PHP, JS, Rust, ...)
  - Pre and post-request script (javascript v8 engine)

- **To improve**
  - Auto-completion on env file variables

### Ideas (will think about it later)

- Base URL on collection

## Documentation

> [!NOTE]
> A documentation will soon be published in the wiki section

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
| [request](https://github.com/seanmonstar/reqwest)                                                                                          | 0.11.24           | Send requests                                                                          |
| [ratatui](https://github.com/ratatui-org/ratatui)                                                                                          | 0.26.1            | Terminal UI framework                                                                  |
| [crossterm](https://github.com/crossterm-rs/crossterm)                                                                                     | 0.27.0            | Terminal Backend                                                                       |
| [tui-big-text](https://github.com/joshka/tui-big-text)                                                                                     | 0.4.1             | Display big texts. Only used for displaying ATAC in the homepage.                      |
| [tui-tree-widget](https://github.com/EdJoPaTo/tui-rs-tree-widget)                                                                          | 0.19.0            | Display tree-like lists. Used for displaying the collections.                          |
| [tui-textarea](https://github.com/rhysd/tui-textarea)                                                                                      | 0.4.0             | Text area that handle a lot of features. Used for editing request body.                |
| [throbber-widgets-tui](https://github.com/arkbig/throbber-widgets-tui)                                                                     | 0.4.1             | Display loading UI elements. Used when request is pending.                             |
| [syntect](https://github.com/trishume/syntect)                                                                                             | 5.2.0             | Syntax highlighting                                                                    |
| [serde](https://github.com/serde-rs/serde) & [serde_json](https://github.com/serde-rs/json)                                                | 1.0.197 & 1.0.144 | Serialize & Deserialize application data into JSON files                               |
| [toml](https://github.com/toml-rs/toml)                                                                                                    | 0.8.10            | Serialize & Deserialize application config files                                       |
| [envfile](https://github.com/pop-os/envfile)                                                                                               | 0.2.1             | Deserialize application environment files                                              |
| [My fork](https://github.com/Julien-cpsn/postman-collection-rs) of [postman_collection](https://github.com/mandrean/postman-collection-rs) | 0.2.1             | Deserialize Postman collection files                                                   |
| [clap](https://github.com/clap-rs/clap)                                                                                                    | 4.5.0             | Command Line Argument Parser                                                           |
| [tokio](https://github.com/tokio-rs/tokio)                                                                                                 | 1.0.0             | Handle asynchronous requests                                                           |
| [strum](https://github.com/Peternator7/strum)                                                                                              | 0.26.1            | Enum facilities                                                                        |
| [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs)                                                                         | 1.4.0             | Allows for more flexible constants. Mainly used for accessing CLI arguments everywhere |
| [regex](https://github.com/rust-lang/regex)                                                                                                | 1.10.3            | Regex. Using for parsing requests URL                                                  |

### Binary size

The binary file size goes from ~4.5 MB to ~7 MB depending on the platform. I try to keep it the smallest I can.

## Contributors

- [@julien-cpsn](https://github.com/julien-cpsn)

## License

The MIT license for this project can be seen [here](./LICENSE)
