# RUSTPractice

A collection of beginner-friendly Rust examples and exercises covering fundamental Rust concepts, ownership, functions, compound data types, and Rustlings practice.

## Topics covered

- Primitive data types
- Arrays, tuples, slices, and strings
- Functions and expressions
- Ownership and borrowing
- A basic Cargo project
- Rustlings exercises

## Repository structure

| Path | Description |
| --- | --- |
| `hello.rs` | Minimal program that prints a Rust greeting |
| `primitive_data_types/` | Signed, unsigned, floating-point, boolean, and character examples |
| `compund_data_types/` | Arrays, tuples, slices, and strings |
| `functions/` | Functions, parameters, return values, expressions, and BMI calculation |
| `ownership/` | Ownership and borrowing concepts |
| `borrowing_ownership/` | A mutable borrowing example |
| `helloProject/` | A small package created with Cargo |
| `Excersize/rustlings/` | Rustlings exercises and official solutions |

> The repository also contains compiled Windows executables and debug files created while experimenting. These are learning artifacts rather than required source files.

## Prerequisites

Install Rust with [rustup](https://rustup.rs/), then verify the tools:

```bash
rustc --version
cargo --version
```

## Running standalone examples

Compile and run `hello.rs`:

```bash
rustc hello.rs -o hello
./hello
```

Compile and run the primitive type examples:

```bash
rustc primitive_data_types/main.rs -o primitive_types
./primitive_types
```

Compile and run the compound type examples:

```bash
rustc compund_data_types/main.rs -o compound_types
./compound_types
```

Compile and run the function examples:

```bash
rustc functions/main.rs -o functions
./functions
```

On Windows, replace `./hello` with `hello.exe`, and use the corresponding `.exe` filenames.

## Running the Cargo projects

Run the simple Cargo project:

```bash
cd helloProject
cargo run
```

To run another package, change to its directory and use `cargo run`:

```bash
cd ownership
cargo run
```

For example:

```bash
cd borrowing_ownership
cargo run
```

## Rustlings exercises

The Rustlings collection is located at `Excersize/rustlings/`.

```bash
cd Excersize/rustlings
cargo run --bin rustlings -- --help
```

Use the exercise files in `Excersize/rustlings/exercises/` as the starting point and consult `Excersize/rustlings/solutions/` only when needed.

## Notes

This repository is primarily for learning and experimentation. Example filenames and some directories intentionally reflect the practice journey, including misspellings such as `Excersize` and `compund_data_types`.
