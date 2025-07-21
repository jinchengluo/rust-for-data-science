# PyO3

This page summarize the basics and essential commands of PyO3. You can check [PyO3 user guide](https://pyo3.rs/main/) if more information is necessary.

PyO3 is a Rust library that enables you to write native Python modules in Rust or to embed Python code inside Rust. It acts as a bridge between Rust’s performance and safety and Python’s usability and ecosystem. PyO3 allows a much more efficient performance-wise :

- **Performance**:	Rust runs much faster than Python, especially for compute-intensive tasks like numerical loops, ML algorithms, or large simulations.
- **Memory Safety**: Rust guarantees safety at compile time—no segfaults or memory leaks. You get the safety of Rust and the flexibility of Python.
- **Seamless Integration**: You can write Rust functions and call them like regular Python functions using import.
- **Great for Extensions**: Perfect for writing Python extensions in Rust instead of C/C++, thanks to safer abstractions.
- **No GIL-holding Threads**: Rust can release Python’s Global Interpreter Lock (GIL) for multi-threaded performance gains.

## Summary

1. [Prerequisites and installation](#1-prerequisites-and-installation)
2. [Python modules](#2-python-modules)
3. [Python functions](#3-python-functions)
4. [Python classes](#4-python-classes)
5. [Python object types](#5-python-object-types)
6. [Quick reference cheat sheet](#6-quick-reference-cheat-sheet)

## 1. Prerequisites and installation [*](https://pyo3.rs/main/getting-started.html)

### Installation

First, make sure you have Rust installed on your system. The minimum required Rust version is `1.74`. If you can run `rustc --version` and the version is new enough you're good to go!

To use PyO3, you need at least Python `3.7`. While you can simply use the default Python interpreter on your system, it is recommended to use a virtual environment.

To use PyO3 and commute from Rust to Python, you must install `maturin` :
```bash
# pip
pip install maturin
# uv
uv tool install maturin
```
You can check the version with `maturin --version`

### Configuration

If you want to start a **new** project from scratch using PyO3, instead of using `cargo new` you can launch `maturin new` :
```
maturin new -b pyo3 <your_project_name>
```

It will initialize a new library Rust package with the `Cargo.toml` correctly configured with PyO3 dependency. It even gives an example of a first `lib.rs` file to start with.

If you want to add maturin to your **existing** Python project **THAT CONTAINS NO RUST YET** :
```
maturin init -b pyo3
```

It will add the necessary files to your project (Cargo.toml, etc..).

If you are already working on a Rust project and wish to use PyO3, you must add the following in your `Cargo.toml` : 
```toml
[lib]
# The name of the native library. This is the name which will be used in Python to import the library. If you change this, you must also change the name of the `#[pymodule]` in `src/lib.rs`.
name = "pyo3_example"
# "cdylib" is necessary to produce a shared library for Python to import from.
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.25.0", features = ["extension-module"] } # Add to your dependencies
```

## 2. Python modules [*](https://pyo3.rs/main/module.html)

You can create a module using `#[pymodule]`. Example :
```rust
use pyo3::prelude::*;

[...]

/// This module is implemented in Rust.
#[pymodule(name="custom_name")]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // code
    Ok(())
}
```
The field `name` of pymodule is optional but allows to custom the name of your library. Otherwise, its name will be the pymodule function name. (Don't forget to remove the parenthesis if you don't use the field `name`)

The function `double()` will be exported to your Rust library and can be used in Python.

You will only have to import the library to Python using `import custom_name`.

## 3. Python functions [*](https://pyo3.rs/main/function.html)

### Define your function 

The `#[pyfunction]` attribute is used to define a Python function from a Rust function. Once defined, the function needs to be added to a module.

```rust
use pyo3::prelude::*;

#[pyfunction]
fn my_function(arg1: type1, ...) -> return_type {
    // code
}
```

If you want to return a Python object from your Rust function, you have to precise it in the return_type and the return is different. We use `Ok(())` : 
```rust
use pyo3::prelude::*;

#[pyfunction]
fn my_function(arg1: type1, ...) -> PyResult<(return_type)> {
    // code 
    Ok((return_object))
}
```

### Signature

The `#[pyo3(signature = (...))]` attribute is used to explicitly define the Python function signature for a Rust function exposed to Python using `#[pyfunction]`.

For example :
```rust
#[pyfunction(signature = (x, y=10))]
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

And in your Python code :
```python
import my_rust_module

my_rust_module.add(2)           # 12 (y defaults to 10)
my_rust_module.add(2, 5)        # 7
my_rust_module.add(x=1, y=2)    # 3
```

Explicitly defining the Python signature is usefull because Rust functions don't support default arguments or keyword arguments like PYthon does. You can also specify default values with it.

### Add functions to your module

To export a function to Python, you have to write this in your pymodule (where `m` is the `&Bound<'_, PyModule>`) :
```rust
m.add_function(wrap_pyfunction!(my_func_to_import, m)?);
```

## 4. Python classes [*](https://pyo3.rs/main/class.html)

You can expose a Rust struct as a Python class using the #[pyclass] attribute.

### Define your class

Use `#[pyclass]` to mark the struct, and `#[pymethods]` to implement the constructor (`#[new]`), instance methods, and optional `__repr__`:

For example : 
```rust
use pyo3::prelude::*;

#[pyclass]
pub struct Counter {
    value: i32,
}

#[pymethods]
impl Counter {
    #[new]
    fn new() -> Self {
        Self { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get(&self) -> i32 {
        self.value
    }

    fn set(&mut self, val: i32) {
        self.value = val;
    }

    fn __repr__(&self) -> String {
        format!("Counter(value={})", self.value)
    }
}
```

You can also use `#[getter]` to define getters if your class has more than one field. The same goes for `#[setter]`.

More convenient, setting a field with `#[pyo3(get, set)]` the line above will assume that you already have a getter and setter.

If you wish to add class methods or static methods, you have to respectively add `#[classmethod]` or `#[staticmethod]` above your method in the `#[pymethods]`.

### Add the class to your module

To export a function to Python, you have to write this in your pymodule (where `m` is the `&Bound<'_, PyModule>`) :
```rust
m.add_class::<your_class_to_import>()?;
```

Then in Python : 
```python
from my_extension import Counter
c = Counter()
```

## 5. Python object types [*](https://pyo3.rs/v0.25.1/types.html)

### GIL & Python Context (Python<'py>)

**What it is** : A token representing that the Python GIL (Global Interpreter Lock) is held — necessary for safe Python API calls.

**Where it appears**: In #[pymodule], #[pyfunction], and methods interacting with Python.

```rust
#[pyfunction]
fn func(_py: Python<'_>, arg1: type1, ...) -> PyResult<(...)> {
    // code
}
```

### Smartt pointer wrappers

PyO3 uses **smart pointers** to safely represent and manage Python objects in Rust. These are essential because Python uses reference counting and dynamic typing, which Rust doesn't natively support.

- #### `Bound<'py, T>`

This is a **reference** to a Python object, tied to the GIL lifetime `'py`. It is used when you are working with a Python object while the GIL is held and is the best for performance and safety when you don't need to store the object long-term.

Should be used inside a #[pyfunction], #[pymodule], or GIL-locked block (Python::with_gil), when passing or modifying a Python object.

Example : 
```rust
fn print_string_length(py: Python<'_>, obj: &Bound<'_, PyString>) {
    println!("Length: {}", obj.len().unwrap());
}
```

- #### `Py<T>` (algo `PyObject`or `Py<Pyany>`)

This is an owning smart pointer to a Python object. It can outlive the GIL and be stored in Rust structs, passed around, or kept for later. You must borrow it with the GIL to use it (via `.as_ref(py)`).

Should be ussed if you want to save or return a Python object outshide the function where the GIL is available.

Example :
```rust
#[pyclass]
struct MyWrapper {
    obj: Py<PyAny>, // Can hold any Python object
}

#[pymethods]
impl MyWrapper {
    #[new]
    fn new(obj: Py<PyAny>) -> Self {
        Self { obj }
    }

    fn print_type(&self) {
        Python::with_gil(|py| {
            let obj_ref = self.obj.as_ref(py);
            println!("Type: {}", obj_ref.get_type().name().unwrap());
        });
    }
}
```

### Python Types: `T` in `Py<T>`, `Bound<'py, T>`

The `T` inside smart pointers like `Py<T>` or `Bound<'py, T>` represents what Python object you are referring to. Here are the most common and useful types :

| Python Type        | PyO3 Rust Equivalent          | Description                                     |
| ------------------ | ----------------------------- | ----------------------------------------------- |
| `str`              | `PyString`                    | For working with Python strings                 |
| `list`             | `PyList`                      | For sequences of Python objects                 |
| `dict`             | `PyDict`                      | For Python dictionaries                         |
| `tuple`            | `PyTuple`                     | Immutable sequences                             |
| `float`            | `PyFloat`                     | Python float                                    |
| `bool`             | `PyBool`                      | Python boolean                                  |
| any                | `PyAny`                       | Used when you don’t know or care about the type |
| user-defined class | `MyClass` (your `#[pyclass]`) | Access custom classes from Rust                 |

**Note** : the function `extract::<T>()` attempts to convert a Python object to a Rust type.

Example : 

```rust
#[pyfunction]
fn first_element(py: Python<'_>, list: &Bound<'_, PyList>) -> PyResult<isize> {
    let val = list.get_item(0).extract::<isize>()?;
    Ok(val)
}
```

### Type Conversions Between Rust & Python

PyO3 provides automatic conversion between Rust types and Python types. These conversions use the traits `FromPyObject`, `IntoPy`, and `ToPyObject`.

| Python Type    | Rust Arg Type                 | Rust Return Type         | Notes                            |
| -------------- | ----------------------------- | ------------------------ | -------------------------------- |
| `int`, `float` | `i32`, `i64`, `f64`, etc.     | same                     | Simple numbers                   |
| `str`          | `&str`, `String`, `&PyString` | `String`, `Py<PyString>` | Strings can be borrowed or owned |
| `list[T]`      | `Vec<T>`, `&PyList`           | `Vec<T>`, `Py<PyList>`   | Ideal for sequence data          |
| `dict[K, V]`   | `HashMap<K, V>`, `&PyDict`    | `HashMap<K, V>`          | Key-value pairs                  |
| `tuple`        | `(T1, T2, ...)`               | tuple types in Rust      | Length must match                |
| `Optional[T]`  | `Option<T>`                   | `Option<T>`              | For optional arguments           |
| `Any object`   | `&PyAny`, `Py<PyAny>`         | Used with `extract()`    | Generic object access            |


### Example and summary

Here is a program that accepts a Python list, processes it in Rust and then returns a Python list : 
```rust
#[pyfunction]
fn double_list(py: Python<'_>, input: &PyList) -> PyResult<Py<PyList>> {
    let list: Vec<i32> = input.extract()?;
    let doubled: Vec<i32> = list.iter().map(|x| x * 2).collect();
    Ok(PyList::new(py, &doubled).into())
}
```

- Use Python<'py> to ensure you're holding the GIL.
- Use Bound<'py, T> for GIL-bound, efficient Python object access.
- Use Py<T> for persistent object references outside the GIL.
- Choose concrete types (PyList, PyDict, etc.) when working with Python built-ins.
- Convert between Python and Rust types via FromPyObject, IntoPy<PyObject>, or direct references.


## 6. Quick Reference Cheat Sheet

| Feature / Macro                  | Description                                                                  | Example / Notes                                                                                  |
|----------------------------------|------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `#[pymodule]`                    | Declares a Python module from a Rust function                                | Defines the Python entry point: `import mymodule`                                                |
| `#[pyfunction]`                  | Exposes a Rust function to Python                                            | Must be registered using `wrap_pyfunction!` inside `#[pymodule]`                                 |
| `wrap_pyfunction!(...)`         | Macro to wrap and register a `#[pyfunction]`                                 | Example: `m.add_function(wrap_pyfunction!(my_func, m)?)?;`                                       |
| `#[pyclass]`                     | Exposes a Rust `struct` as a Python class                                    | Requires `#[pymethods]` to define methods and properties                                          |
| `#[pymethods]`                   | Defines methods for a `#[pyclass]`                                           | Supports `__new__`, `__repr__`, instance methods, class/static methods                           |
| `Python<'py>`                   | Python interpreter token (needed for object creation and Python interaction) | Used with `Python::with_gil(|py| { ... })`                                                       |
| `PyResult<T>`                   | Return type for functions exposed to Python                                  | Allows raising Python exceptions using `Err(PyErr::new::<...>)`                                 |
| `PyModule`, `PyAny`, `PyObject` | Types to manipulate Python objects and modules                               | Used to call Python functions or access attributes from Rust                                     |
| `py.run(...)`                   | Executes Python code from Rust                                               | Requires access to Python context via `Python::with_gil`                                         |
| `#[cfg(feature = "extension-module")]` | Ensures correct compilation for building Python extension           | Required for producing a `.so` / `.pyd` module loadable from Python                              |
| `m.add(...)`, `m.add_class(...)`| Adds objects, functions, or classes to the module                            | Use inside the `#[pymodule]` function                                                            |
| `PyErr`, `exceptions::PyValueError` | Handle and raise Python-style exceptions                                | Used for safe error messaging between Rust and Python                                            |
| `PyReadonlyArray` / `PyArray`   | Work with `numpy.ndarray` from Python using `ndarray` crate                  | Essential for data science, efficient numeric operations from Rust                               |
| `maturin`                       | Tool to build and install Rust-based Python extensions                       | Run `maturin develop` in a virtualenv for local dev                                               |
| `setuptools-rust`               | Alternative to `maturin` for packaging via `setup.py`                        | More flexible, but more complex                                                                  |

- Always use `maturin develop` to test Rust modules inside a Python virtual environment.
- Use `PyReadonlyArray` for safe and fast read access to NumPy arrays.
- Use `cdylib` crate type in `Cargo.toml` when targeting Python.
- Enable `extension-module` feature in `pyo3` for correct linkage.
- Use `PyErr::new::<PyValueError, _>("message")?` to raise errors to Python.

### Exemple of a lib.rs
```rust
use pyo3::prelude::*;

#[pyfunction]
fn square(x: usize) -> usize {
    x * x
}

#[pymodule]
fn rust_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(square, m)?)?;
    Ok(())
}
```

## Source

- [PyO3 user guide (PyO3 official page)](https://pyo3.rs/main/function.html)