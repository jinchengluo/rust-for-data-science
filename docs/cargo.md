# Cargo

This page summarize the basics and essential commands of Cargo. You can check [The Cargo Book](https://doc.rust-lang.org/cargo/index.html) if more information is necessary.

Cargo is the Rust package manager. Cargo downloads your Rust package’s dependencies, compiles your packages, makes distributable packages, and uploads them to crates.io, the Rust community’s package registry.

## Summary

1. [Create a new package](#1-create-a-new-package)
2. [Compile a package](#2-compile-a-package)
3. [Add dependencies](#3-add-dependencies)
4. [Run tests](#4-run-tests)
5. [Conventional package layout](#5-conventional-package-layout)
6. [Usefull Cargo commands](#6-usefull-cargo-commands)

## 1. Create a new package

```bash
cargo new <package_name>
cargo new <package_name> --lib # to make a library
cargo new <package_name> --vcs none # no git init
```
By default, a new package is in `--bin`, a binary program.

For a binary package :
```bash
package_name
├── Cargo.toml
└── src
    └── main.rs
```

## 2. Compile a package

For non-library packages :
```bash
cargo build # compiles and generates a binary crate
./target/debug/<project_name> # runs

or

cargo run # compiles and runs
```

For library packages :
```bash
cargo build
maturin develop # generates a python library to import
```

You can add `--release` to `cargo build` for an optimized compilation and the executable file will be found at `/target/release/` instead of `/target/debug/`

## 3. Add dependencies

To use functions and types that are not directly implemented in Rust, you must import dependencies with the following command :
```bash
cargo add <dependency>
```

To update a dependency
```bash
cargo update <dependency>
cargo update # updates all dependencies
```

You can add for example : `pyo3`, `linfa`, `linfa-clustering`, `numpy`, `ndarray`, `rand`.

The importation can be found in the `Cargo.toml` file. If necessary, the version of a dependency can be manually changed.

## 4. Run tests

```bash
cargo test # runs all tests
cargo test <sth> # runs all tests that contains <sth> in their name
```

## 5. Conventional package layout

```bash
my_project
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

 - `Cargo.toml` and `Cargo.lock` are stored in the root of your package (package root).
 - Source code goes in the `src` directory.
 - The default library file is `src/lib.rs`.
 - The default executable file is `src/main.rs`.
    - Other executables can be placed in `src/bin/`.
 - Benchmarks go in the `benches` directory.
 - Examples go in the `examples` directory.
 - Integration tests go in the `tests` directory.

## 6. Usefull Cargo commands

| Command                            | Description                                                                                      |
|------------------------------------|--------------------------------------------------------------------------------------------------|
| `cargo new my_project`             | Create a new binary project (`main.rs`) in `my_project` directory                                 |
| `cargo new --lib my_lib`           | Create a new library project (`lib.rs`)                                                           |
| `cargo init`                       | Initialize Cargo in an existing folder (creates `Cargo.toml`)                                     |
|                                    |                                                                                                  |
| `cargo build`                      | Compile the project (debug mode by default)                                                       |
| `cargo build --release`           | Compile with optimizations (slower to build, faster to run)                                      |
| `cargo check`                      | Type-check without producing a binary (fast, great for IDEs and early error detection)            |
| `cargo run`                        | Build and run the project                                                                         |
| `cargo test`                       | Run unit/integration tests                                                                        |
| `cargo doc --open`                | Generate documentation and open it in the browser                                                 |
| `cargo clean`                      | Delete `target/` directory (cleans compiled artifacts)                                            |
|                                    |                                                                                                  |
| `cargo add package_name`          | Add a dependency (requires `cargo-edit` installed)                                                |
| `cargo rm package_name`           | Remove a dependency                                                                               |
| `cargo update`                     | Update all dependencies to the latest versions allowed by `Cargo.toml`                           |
| `cargo upgrade`                    | Upgrade dependencies to the latest **available** versions (requires `cargo-edit`)                |
| `cargo tree`                       | Display dependency tree (requires `cargo-tree`)                                                   |
| `cargo metadata`                   | Outputs project structure in JSON (useful for tooling and automation)                            |
|                                    |                                                                                                  |
| `cargo publish`                    | Publish your crate to [crates.io](https://crates.io)                                              |
| `cargo login`                      | Authenticate with crates.io                                                                      |
| `cargo package`                    | Create a distributable `.crate` archive                                                           |
| `cargo install crate_name`        | Install a Rust binary crate globally (e.g. CLI tools)                                             |
| `cargo uninstall crate_name`      | Remove a globally installed crate                                                                 |

| File               | Description                                                                 |
|--------------------|-----------------------------------------------------------------------------|
| `Cargo.toml`       | Manifest file: project name, version, authors, dependencies, metadata       |
| `Cargo.lock`       | Auto-generated: exact versions used for reproducible builds (don’t edit!)   |
| `src/main.rs`      | Main file for binary crates                                                  |
| `src/lib.rs`       | Main file for library crates                                                 |
| `target/`          | Build artifacts and intermediate files go here                              |

## Source

- [The Cargo Book (Rust official page)](https://doc.rust-lang.org/cargo/index.html)