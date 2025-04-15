---
title: Rust Testing
author: Dávid Seres
date: 2025-04-12
extensions: 
    - terminal
styles: {}
---

# Introduction

Dávid Seres

freelancer

dseres001@gmail.com

https://github.com/dseres/rust_test_training

Testing in Rust will be introduced by developing a small application.

---

# The application

A simple image processing application. Image will be:

```txt
92 213 101
151 208 193
88 121 36
```
The app will show the value of darkest and lightest pixel. In this case they are 14 and 224.


```bash
# Create an application:
cargo new --bin --edition 2024 --name rtt -v rust_test_training
cd rust_test_training
cargo run
```

Add the following dependency to cargo.toml:
```toml
[dependencies]
reqwest = { version="0.12.15", features = ["blocking"]}
```

---

# First version

```rust
use std::error::Error;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut image : Vec<Vec<u8>> = Vec::new();
    let input = reqwest::blocking::get("https://raw.githubusercontent.com/dseres/rust_test_training/refs/heads/master/examples/image.txt")?.text()?;
    for input_line in input.trim().lines() {
        let mut line: Vec<u8> = Vec::new();
        for pixel_str in input_line.trim().split(" ") {
            let pixel = pixel_str.parse::<u8>().unwrap();
            line.push(pixel);
        }
        image.push(line);
    }

    let min = image.iter().map(|line| line.iter().min().unwrap()).min().unwrap();
    let max = image.iter().map(|line| line.iter().max().unwrap()).min().unwrap();

    println!("{min},{max}");
    return Ok(());
}
```

```terminal8
cargo run --example slide3
```