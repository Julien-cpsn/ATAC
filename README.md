ATAC ⚔📩
===

[![Rust](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml/badge.svg)](https://github.com/Julien-cpsn/ATAC/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Release](https://img.shields.io/github/v/release/julien-cpsn/ATAC?link=https%3A%2F%2Fgithub.com%2FJulien-cpsn%2FATAC%2Freleases%2Flatest)

## Description

ATAC is **A**rguably a **T**erminal **A**PI **C**lient. It is based on well known clients such as Postman, Insomnia or even Bruno, but inside your terminal without any specific graphical environment needed.

The philosophy of ATAC is to be free, account-less and offline for now and forever.

## How to run

> First, make sure your rust version is at least 1.76

Simply clone the repository and use :

```bash
cargo run
```

## Documentation

A documentation will soon be published in the wiki section

## TODO

### v1.0.0

- **To add**
  - Headers management
  - Cookies management
  - Proxy configuration
  - Postman & Insomnia import
  - Async request support
  - File based memory (for now the memory is blank at every program startup)
  - Body & result syntax highlighting
  - Create a repo wiki
  - Document whole code

- **To improve**
  - Create collection
  - Rename request
  - Keymap help on bottom right

### v2.0.0

- **To add**
  - Environments (maybe .env files?)
  - keymap configuration (via a config file)
  - Export a request to other code formats (curl, PHP, JS, Rust, ...)
  - Pre and post-request script

## Contributors

- [@julien-cpsn](https://github.com/julien-cpsn)