# Rust

This page is a summary of the basics of Rust programming language. If you want more information et and more complete guide for Rust, you can take a look in [The "Book"](https://doc.rust-lang.org/book/).

### Why Rust?
 - It is one of the fastest-growing programming languages in the world
 - Learning Rust makes it easier to pick up other languages like Java, Python, C++, and C#, because the syntax is similar.
 - Rust is very fast
 - Rust requires less memory compared to many other languages
 - Rust is used to build web servers, creating games, operating systems, and much more!

In each section, the title will be followed by a star (*) that redirects to the "Book".

1. [Prerequisites and installation](#1-prerequisites-and-installation-)
2. [Variables and constants](#2-variables-and-constants)  
3. [Operators](#3-operators)
4. [If..else and loops](#4-ifelse-and-loops)
5. [Functions](#5-functions)
6. [Scope](#6-scope)

## 1. Prerequisites and installation [*](https://doc.rust-lang.org/book/ch01-01-installation.html)
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh # on Linux
$ xcode-select --install # on macOS
```
You should find a `Rust is now installed now. Great!` message once installed correctly. You can also run `rustc --version` to check.

To update your current local Rust version, launch this command :
```bash
$ rustup update
```

## 2. Variables and constants

Variables are containers for storing data values, like numbers and characters.
```rust
let variable = value;
let variable : type = value; // If you want to have a precise data type which is optional but it is sometimes recommended to do
```

A variable value can not be changed unless the attribute `mut` is placed after `let` (which means mutable) :
```rust
let x = 5;
x = 1000; // Error

let mut y = 5;
y = 1000; // value is now changed
```

There are several data types in Rust whose the main ones are : 
```rust
let integer: i32 = 5;          // integer
let float: f64 = 5.99;    // float
let letter: char = 'D';    // character
let boolean: bool = true;     // boolean
let string: &str = "Hello";  // string
```

You can define constants :
```rust
const CONST : type = value;
```
**Disclaimer** : You must specify the type when creating a constant.

## 3. Operators

**Arithmetic Operators** : `+` (addition), `-` (substraction), `*` (multiplication), `/` (division), `%` (modulus)

**Assigment Operators** : `=` (value assignment), any arithmetic operator followed by a `=` 

**Comparison operators** : `==`, `!=`, `>`, `<`, `>=`, `<=`

## 4. If..else and loops

```rust
if condition1 {
    // code
} else if conditon2 {
    // code
} else {
    // code
}
```

```rust
let var = val;
match var {
    val1 => //instruction1,
    val2 => //instruction2,
    val3 => //instruction3,
    ...
    _ => // else
} // Similar to if statements but 
```

```rust
for iterator in lower_bound..upper_bound {
    // code
}
```
`iterator` goes from `lower_bound` to `upper_bound - 1`. If you wish to include upper_bound, use `..=`.

```rust
while condition {
    // code
}
```

## 5. Functions

```rust
fn function_name(arg1: type1, arg2: type2, ...) -> return_type {
    // code to be executed
    return object_to_return
}
```
The `return_type` can be omit but it is recommend to specify it in order to avoid type errors. The `return` keyword can also be omit and the object to return should not end with a semicolon and must be at the end of the function.

A common function is the print macro :
```rust
let variable = value;
println!("Variable value is here : {}", variable)
```

## 6. Scope

A scope refers to where a varaible is allowed to be used. A varible only lives inside the block where it as created. A block is anything inside curly braces  `{ }`.

Example :

```rust
fn func() {
    let message : &str = "Hello datacrafter";
    println!("{}", message);
}

func();
println!("{}", message); // Error - the variable was local to the function block.
```

```rust
let x : i32 = 5;
let x : i32 = 10; // Error - name already used in this scope
```

```rust
let x : i32 = 5;
for i in 0..3 {
    let x : i32 = 42;
    println!("{}", x); // prints 42
}
println!("{}", x); // prints 5
```

## Source
- [The "Book" (Rust official page)](https://doc.rust-lang.org/book/)
- [Rust tutorial (W3Schools)](https://www.w3schools.com/rust/rust_intro.php)"