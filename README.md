# Rust for Data Science

This is a guide for Data Scientists and Machine Learning learners that seek to learn **Rust** from scratch alongside with **Cargo**, its package manager tool, and **PyO3**, a Rust library that gives developers a direct access to manually written Rust functions from Python files with simple imports.

This guide is divided in four parts : 
1. [**Basics of Rust**](./docs/rust.md)
2. [**Basics of Cargo**](./docs/cargo.md)
3. [**How to use PyO3**](./docs/pyo3.md)
4. [**Applications and tests**](./docs/applications.md)

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
uv pip install -r requirements.txt
uv tool install maturin
```

### Compile
```bash
maturin develop --develop
```
Then run your Python code.

**Disclaimer**: if you work in a notebook, you must restart your kernel to access your library update from maturin.
