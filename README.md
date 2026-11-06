# RUSTPractice

A small Rust learning repository with beginner-friendly examples and practice programs.

This project contains a few simple Rust scripts and a small Cargo project to help explore Rust basics such as:

- printing to the console
- using primitive data types
- running Rust programs with `rustc`
- building a small Cargo project

## Repository structure

- `hello.rs` - a minimal Rust program that prints a greeting
- `helloProject/` - a Rust project created with Cargo
- `primitive_data_types/main.rs` - examples of integer types and basic data type usage
- `hello.exe`, `hello.pdb`, `main.exe`, `main.pdb` - compiled binaries generated during practice

## Prerequisites

Make sure Rust is installed on your machine:

```bash
rustc --version
cargo --version
```

If Rust is not installed, install it with Rustup:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Run the simple hello example

```bash
rustc hello.rs -o hello
./hello
```

## Run the primitive data types example

```bash
rustc primitive_data_types/main.rs -o main
./main
```

## Run the Cargo project

```bash
cd helloProject
cargo run
```

## Notes

This repository is meant for learning and experimentation. It is intentionally simple and focused on basic Rust syntax and project setup.
