eui-no-std 
============
[![crates.io](https://img.shields.io/crates/v/eui-no-std.svg)](https://crates.io/crates/eui-no-std)
[![Build Status](https://travis-ci.org/vagola/eui-no-std.svg?branch=master)](https://travis-ci.org/vagola/eui-no-std)
[![codecov](https://codecov.io/gh/vagola/eui-no-std/branch/master/graph/badge.svg)](https://codecov.io/gh/vagola/eui-no-std)

EUI-48 and EUI-64 no-std implementation using heapless. 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
eui-no-std = "0.2"
```

Serde support can be enabled using features:

```toml
[dependencies]
eui-no-std = { version = "0.2", default-features = false, features = ["serde"] }
```

## Example

```rust
use eui::Eui48;
use eui::Eui64;

let eui48 = Eui48::from(85204980412143);
let eui64 = Eui64::from(eui48);
    
assert_eq!(eui48.to_string(), "4d7e54972eef");
assert_eq!(eui64.to_string(), "4d7e540000972eef");
```
