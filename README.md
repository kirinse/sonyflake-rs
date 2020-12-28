# sonyflake-rs

[![CI](https://github.com/bahlo/sonyflake-rs/workflows/CI/badge.svg)](https://github.com/bahlo/sonyflake-rs/actions?query=workflow%3ACI)
[![Audit](https://github.com/bahlo/sonyflake-rs/workflows/Audit/badge.svg)](https://github.com/bahlo/sonyflake-rs/actions?query=workflow%3AAudit)
[![crates.io](https://img.shields.io/crates/v/sonyflake.svg)](https://crates.io/crates/sonyflake)
[![docs.rs](https://docs.rs/sonyflake/badge.svg)](https://docs.rs/sonyflake/)



A Rust implementation of [Sonyflake](https://github.com/sony/sonyflake), a distributed unique ID generator inspired by [Twitter's Snowflake](https://blog.twitter.com/2010/announcing-snowflake).

## Install

Add the following to your `Cargo.toml`:
```toml
[dependencies]
sonyflake = "0.1.0"
```

## Quickstart

```rust
use sonyflake::Sonyflake;

let mut sf = Sonyflake::new().unwrap();
let next_id = sf.next_id().unwrap();
println!("{}", next_id);
```
