# TLD Download
[![Linux Arm7](https://github.com/marirs/tld-download-rs/actions/workflows/linux_arm.yml/badge.svg)](https://github.com/marirs/tld-download-rs/actions/workflows/linux_arm.yml)
[![Linux x86_64](https://github.com/marirs/tld-download-rs/actions/workflows/linux_x86_64.yml/badge.svg)](https://github.com/marirs/tld-download-rs/actions/workflows/linux_x86_64.yml)
[![macOS](https://github.com/marirs/tld-download-rs/actions/workflows/macos.yml/badge.svg)](https://github.com/marirs/tld-download-rs/actions/workflows/macos.yml)

Downloads tld suffixes to a file

#### Requirements

- Rust 1.75+

## Usage

You can include this in your Cargo.toml file:
```toml
[dependencies]
tld_download = "0.1.4"
```

If you want to use it with the inbuilt public suffix db; then:
```toml
[dependencies]
tld_download = { version = "0.1.4", features = ["with-db"] }
```

and then

```rust
use tld_download::from_db;

fn main () {
    let db = from_db();
    assert!(!db.is_empty());
}
```
---
License: MIT
