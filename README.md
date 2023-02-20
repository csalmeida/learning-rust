# Learning rust

This repo stores some of the programs made while following the documentation available on the [Rust Programming Language book](https://doc.rust-lang.org/book/second-edition/ch01-00-introduction.html) (second edition).

## Setup

Install Rust on macOS and Linux using:

```
$ curl https://sh.rustup.rs -sSf | sh
```

Follow the installation instructions on the official guide for Windows.

## Running Rust programs

In order to run a program use `rustc filename.rs` like so:

```
$ rustc main.rs
```

To run programs using Cargo `cd` into any project that has a `Cargo.toml` file and run:

```
$ cargo run
```

## Offline Rust Book

To learn on the go Rust allows the learning documentation to be accessed offline. However, some of the lessons require Cargo dependencies to be installed which would require an internet connection.

```bash
rustup docs --book
```
