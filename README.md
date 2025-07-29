# Rust for Data Science

This is a guide for Data Scientists and Machine Learning learners that seek to learn **Rust** from scratch alongside with **Cargo**, its package manager tool, and **PyO3**, a Rust library that gives developers a direct access to manually written Rust functions from Python files with simple imports.

This guide is divided in four parts : 
1. [**Basics of Rust**](./docs/rust.md)
2. [**Basics of Cargo**](./docs/cargo.md)
3. [**How to use PyO3**](./docs/pyo3.md)
4. [**Applications and tests**](./docs/applications.md)

## You are in the `main` branch

To check the **Applications** part and do the exercices, you must go to the `TODO` branch with :
```bash
git switch TODO
```

In the `TODO` branch, certain files contain a TODO section that you must fill.

## Usage

### Clone project
```bash
git clone git@github.com:jinchengluo/rust_for_data_science.git
```

### Install Rust and Cargo
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Check with `rustc --version` and `cargo --version`

### Install uv
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### Create `.venv`
```bash
uv venv # at the root of the repo
source .venv/bin/activate
```

### Install dependencies
```bash
uv sync
```