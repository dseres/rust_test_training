---
title: Rust Testing
author: Dávid Seres
date: 2025-04-12
extensions: []
styles: {}
---

# Introduction

Dávid Seres

freelancer

dseres001@gmail.com

https://github.com/dseres/rust_test_training

Testing in Rust will be introduced by developing a small application.

---

# Slide 2 - The application

A simple image processing application. 

```rust
let _image = [
    [124_u8, 65_u8 , 32_u8], 
    [91_u8, 70_u8 , 14_u8], 
    [45_u8, 88_u8 , 224_u8], 
];
```
The app will show the value of darkest and lightest pixel. In this case they are 14 and 224.

```bash
# Create an application:
cargo new --bin --edition 2024 --name rtt -v rust_test_training
cd rust_test_training
cargo run
```

