# Rust-QRCLI
A [command-line interface](https://en.wikipedia.org/wiki/Command-line_interface) for the [Rust-QR](/MilkFather/rust-qr) repository

[![Rust](https://github.com/MilkFather/rust-qrcli/actions/workflows/rust.yml/badge.svg)](https://github.com/MilkFather/rust-qrcli/actions/workflows/rust.yml)

## Features
* A simple but powerful syntax
* Supports image saving

## Usage
Simplist usage:
```
qrcli "hello world"
```
Further options include encoding control, error correction level control, QR code version control and output image sizing control. Run
```
qrcli --help
```
for the full documentation.

## Build by yourself
Building the project is simple. You need to have the basic Rust toolchain (rustc, cargo, etc.) installed.

First, download the whole repository by
```
git fetch --recursive "https://github.com/MilkFather/rust-qrcli.git"
```

Then go to the folder and run
```
cargo build
```
for development profile or
```
cargo build --release
```
for release profile.
