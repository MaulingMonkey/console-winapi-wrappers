# maulingmonkey-console-winapi-wrappers

Unstable wrapper API for [winapi](https://docs.rs/winapi/)'s [Console Functions](https://docs.microsoft.com/en-us/windows/console/console-functions)

<!--
[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/maulingmonkey-console-winapi-wrappers.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/maulingmonkey-console-winapi-wrappers)
[![crates.io](https://img.shields.io/crates/v/maulingmonkey-console-winapi-wrappers.svg)](https://crates.io/crates/maulingmonkey-console-winapi-wrappers)
[![docs.rs](https://docs.rs/maulingmonkey-console-winapi-wrappers/badge.svg)](https://docs.rs/maulingmonkey-console-winapi-wrappers)
[![License](https://img.shields.io/crates/l/maulingmonkey-console-winapi-wrappers.svg)](https://github.com/MaulingMonkey/maulingmonkey-console-winapi-wrappers)
[![Build Status](https://github.com/MaulingMonkey/maulingmonkey-console-winapi-wrappers/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/maulingmonkey-console-winapi-wrappers/actions?query=workflow%3Arust)
-->
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/maulingmonkey-console-winapi-wrappers/status.svg)](https://deps.rs/repo/github/MaulingMonkey/maulingmonkey-console-winapi-wrappers) -->



## Quickstart

```toml
# Cargo.toml
[dependencies]
maulingmonkey-console-winapi-wrappers.git = "https://github.com/MaulingMonkey/console-winapi-wrappers"
```

```rust
// src\main.rs
use maulingmonkey_console_winapi_wrappers::*;

fn main() {
    let _ = alloc_console();
    println!("Hello, world!");
}
```



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
