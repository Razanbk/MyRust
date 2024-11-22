# Memory Allocator Library

A `no_std` memory allocator library built in Rust for embedded and low-level systems. This project demonstrates how to implement a custom memory allocator using a bump allocator strategy.

## Features

- **No standard library** (`no_std` support).
- Custom bump allocator implementation.
- Support for `alloc` feature and error handlers.

## Usage

### Prerequisites

1. Install Rust's nightly toolchain:
   ```bash
   rustup install nightly
   rustup override set nightly
2. Ensure you have Cargo and Rust installed:
   ```bash
   rust --version
   cargo --version
3. Build
   ```bash
   cargo build
4. Test
   ```bash
   cargo test
