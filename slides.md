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

---

# The test application

A simple image processing application. An image will be:

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

---

# Prerequisites

Add the following dependency to cargo.toml:
```toml
[dependencies]
reqwest = { version="0.12.15", features = ["blocking"]}
```

```bash
# install dependencies:
sudo apt install -y libyaml-dev libssl-dev pkg-config
```

---

# First version

```rust
use std::error::Error;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let input = reqwest::blocking::get("https://raw.githubusercontent.com/dseres/rust_test_training/refs/heads/master/examples/image.txt")?.text()?;

    let mut image : Vec<Vec<u8>> = Vec::new();
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
    Ok(())
}
```

```txt
$cargo run --example slide4
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/slide4`
```

---

# Refactor

```rust
use std::error::Error;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let input = download()?;

    let image = parse(&input);

    let (min, max) = minmax(&image);

    println!("{min},{max}");
    Ok(())
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut image: Vec<Vec<u8>> = Vec::new();
    for input_line in input.trim().lines() {
        let mut line: Vec<u8> = Vec::new();
        for pixel_str in input_line.trim().split(" ") {
            let pixel = pixel_str.parse::<u8>().unwrap();
            line.push(pixel);
        }
        image.push(line);
    }
    image
}

fn download() -> Result<String, Box<dyn Error>> {
    let input = reqwest::blocking::get("https://raw.githubusercontent.com/dseres/rust_test_training/refs/heads/master/examples/image.txt")?.text()?;
    Ok(input)
}

fn minmax(image: &[Vec<u8>]) -> (u8, u8) {
    let min = image
        .iter()
        .map(|line| line.iter().min().unwrap())
        .min()
        .unwrap();
    let max = image
        .iter()
        .map(|line| line.iter().max().unwrap())
        .max()
        .unwrap();
    (*min, *max)
}
}
```

---

# Add unit tests

```rust
#[cfg(test)]
mod test {

    static TXT : &str = "92 213 101\n224 197 198\n34 230 56";

    #[test]
    fn test_parse() {

        let image: Vec<Vec<u8>> = Vec::from([
            Vec::from([92_u8, 213_u8, 101_u8]),
            Vec::from([224_u8, 197_u8, 198_u8]),
            Vec::from([34_u8, 230_u8, 56_u8]),
        ]);
        assert_eq!(crate::parse(TXT), image)
    }

    #[test]
    fn test_minmax() {
        let image = crate::parse(TXT);
        assert_eq!(crate::minmax(&image), (34_u8, 230_u8), 
            "Min and max should be 34 and 230 on the predefined matrix.");
    }
}
```

```txt
$cargo test --example slide6
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests examples/slide6.rs (target/debug/examples/slide6-19f1aab8c1dd9f8f)

running 2 tests
test test::test_parse ... ok
test test::test_minmax ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# Test params

```bash
cargo test --help
```

E.g.:
```bash
cargo test --example slide6 test_minmax
```

---

# Ignoring tests

```rust
    #[test]
    #[ignore("Unimplemented yet")]
    fn test_download() {
        unimplemented!()
    }
```

Try the following commands:
```bash
cargo test --example slide6 
cargo test --example slide6 -- --include-ignored
```