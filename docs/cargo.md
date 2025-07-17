# Cargo

## Create a new package

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

## Compile a package

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

## Add dependencies

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

## Run tests

```bash
cargo test # runs all tests
cargo test <sth> # runs all tests that contains <sth> in their name
```

## Conventional package layout

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
