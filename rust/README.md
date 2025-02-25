# Rust Course

Welcome to the Rust course! This repository contains all the materials and exercises for learning Rust.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Rust Major Concepts](#rust-major-concepts)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This course is designed to help you learn Rust, a systems programming language focused on safety, speed, and concurrency.

## Installation

To install Rust, follow these steps:

1. Visit the [official Rust website](https://www.rust-lang.org/).
2. Follow the instructions to download and install Rust.

## Getting Started

Once Rust is installed, you can verify the installation by running:

```sh
rustc --version
```

## Rust Major Concepts

### Foundations of Rust
- Why Rust?
  Memory safe, Thread safe low level systems programming language
- Rust Installation
  [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Creating and Running Rust Projects
  Using cargo commands

### Core Concepts
- [IMP] Representing Data with Structs, adding implementations
- Rust datatypes
  Integer, Boolean, Tuples, Floats, Strings, None
- Control flow (if else), Pattern matching (match)
- Macros and println! macro
- Arrays vs Vectors
- Mutable vs Immutable Bindings
- Implicit returns
- Installing External Crates (from crates.io), Cargo.toml file

### [IMP] Ownership and Borrowing
- The Basics of Ownership, reducing unexpected updates
- Introducing the Borrow System
- Understanding stack and heap
- Immutable, Mutable References
- Copy vs Moving values, Copy-able Values

### [IMP] Lifetimes
- Basics of Lifetimes, how long does a binding live?
- Lifetime Annotations using ‘a
- When to omit Lifetime Annotations?

### Enums
- Defining Enums
- [IMP] Pattern Matching
- Struct vs Enums
- The Option Enum
- Unwrapping values

### Mastering Modules in Rust
- Modules
- Refactoring main.rs into Multiple Modules
- Understanding file system hierarchy and super keyword

### Errors and Results
- The Result Enum – Ok (), Err ()
- Pattern Matching on Results
- [IMP] Strings, String Refs, and String Slices
- Propagating errors and Try Operator ‘?’
- Reading and Writing data to files

### Iterators
- Basics of Iterators
- Using For Loops with Iterators
- Iterator Consumers vs Adaptors
- Vector slices
- Collecting Elements from an Iterator using collect()
- [IMP] Understanding function closure syntax
- The Filter Method
- Nested mappings

### Generics and Traits
- The Basics of Generics
- Multiple Generics
- Understanding Traits (like interfaces in Java)
- Generic Structs
- Implementing a Trait

### More Resources
- [https://www.rust-lang.org/learn](https://www.rust-lang.org/learn)
- [https://github.com/rust-lang/rustlings/](https://github.com/rust-lang/rustlings/)
- [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)

[ Note - Feel free to add or modify the bullet points!! ]

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

