# Rust

This page is a summary of the basics of Rust programming language. If you want more information et and more complete guide for Rust, you can take a look in [The Book](https://doc.rust-lang.org/book/).

### Why Rust?
 - It is one of the fastest-growing programming languages in the world
 - Learning Rust makes it easier to pick up other languages like Java, Python, C++, and C#, because the syntax is similar.
 - Rust is very fast
 - Rust requires less memory compared to many other languages
 - Rust is used to build web servers, creating games, operating systems, and much more!

In each section, the title will be followed by a star (*) that makes a reference to The Book if some additional information can be necessary.

## Table of Contents

1. [Prerequisites and installation](#1-prerequisites-and-installation)
2. [Variables and constants](#2-variables-and-constants)  
3. [Operators](#3-operators)
4. [Control flow](#4-control-flow)
5. [Functions](#5-functions)
6. [Scope](#6-scopes)
7. [Ownership](#7-ownership)
8. [Reference and borrowing](#8-references-and-borrowing)
9. [Data structures](#9-data-structures)
    1. [Arrays](#91-arrays)
    2. [Vectors](#92-vectors)
    3. [HashMap](#93-hashmap)
    4. [Structs](#94-structs)
10. [Summary](#10-summary)

## 1. Prerequisites and installation [*](https://doc.rust-lang.org/book/ch01-01-installation.html)

If not already done, install Rust and Cargo with the following command :
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh # on Linux
$ xcode-select --install # on macOS
```
You should find a `Rust is now installed now. Great!` message once installed correctly. You can also run `rustc --version` to check.

To update your current local Rust version, launch this command :
```bash
$ rustup update
```

## 2. Variables and constants [*](https://doc.rust-lang.org/book/ch03-02-data-types.html)

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
let u_integer : usize = 5; // unsigned interger
let float: f64 = 5.99;    // float
let letter: char = 'D';    // character
let boolean: bool = true;     // boolean
let string: &str = "Hello";  // string
let mut mut_string: String = "Hello!" // mutable String
```
**Disclaimer** : use `String` as it is considered like a list (Vec<>) whose size is dynamic.

Tuples are defined using `( )` : `let tuple : (type1, type2, type3, ...) = (x, y, z, ...)`

You can define constants :
```rust
const CONST : type = value;
```
**Disclaimer** : You must specify the type when creating a constant.

## 3. Operators [*](https://doc.rust-lang.org/book/appendix-02-operators.html)

**Arithmetic Operators** : `+` (addition), `-` (substraction), `*` (multiplication), `/` (division), `%` (modulus)

**Assigment Operators** : `=` (value assignment), any arithmetic operator followed by a `=` 

**Comparison operators** : `==`, `!=`, `>`, `<`, `>=`, `<=`

## 4. Control flow [*](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

Conditions using if, elif and else :
```rust
if condition1 {
    // code
} else if conditon2 {
    // code
} else {
    // code
}
```

Match a variable to different value (similar to `switch` in other programming languages) :
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

For loop :
```rust
for iterator in lower_bound..upper_bound {
    // code
}
```
`iterator` goes from `lower_bound` to `upper_bound - 1`. If you wish to include upper_bound, use `..=`.

While loop :
```rust
while condition {
    // code
}
```

## 5. Functions [*](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

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

## 6. Scopes [*](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variable-scope)

A scope refers to where a varaible is allowed to be used. A varible only lives inside the block where it as created. A block is anything inside curly braces  `{ }`.

Example :

```rust
fn func() {
    let message : &str = "Hello datacrafter";
    println!("{}", message);
}

func();
println!("{}", message); // Error: the variable was local to the function block.
```

```rust
let x : i32 = 5;
let x : i32 = 10; // Error: name already used in this scope
```

```rust
let x : i32 = 5;
for i in 0..3 {
    let x : i32 = 42;
    println!("{}", x); // prints 42
}
println!("{}", x); // prints 5
```

## 7. Ownership [*](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

Rust uses "ownership" to manage memory in a safe way. Every value in Rust has an owner. The owner is usually a variable. There are 3 rules to ownership in Rust :

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

For example :
```rust
let x = String::from("Hello");
let y = x; 
println!("{}", x); // -> Error: a no longer owns the value and is now empty
println!("{}", y); // Prints the value owned by y and previously owned by x
```

However the ownership rules only apply when manipulating data structures such as list or array (String is considered as a list). Thus, assigning an interger from an interger variable works and the value is only copied. The same occurs for floats, characters and booleans.
```rust
let x = 5;
let y = x;
println!("{}", x); // Works and prints 5
println!("{}", y); // Works and prints 5, y copied the value inside x
```

For other types such as `String`, the method `.clone()` exists. It copies the data.

## 8. References and borrowing [*](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

Sometimes you want to use a value without taking ownership of it. Rust lets you do this using a reference, this is called a **borrowing**.

You can create a reference using the symbol `&` in front of a variable.
```rust
let x = String::from("Hello");
let y = &x;
println!("{}", x); // Works, prints Hello
println!("{}", y); // Works, prints Hello
```

However, for instance in the previous case, `y` only borrows the value of `x` and `x` still owns it. That means if you change the value of `y`, the same goes for `x` (while assuming that the `mut` keyword was added).

They both point to the same memory emplacement and you can change a value through a reference.
```rust
let mut x = String::from("Hello");
let mut y = &mut x;
y.push_str(" datacrafter!");
println!("{}", x); // Works, prints Hello datacrafter!
println!("{}", y); // Works, prints Hello datacrafter!
```
**Note** : You can only have one mutable reference to a value at a time!

In short : Borrowing helps you reuse values safely, without giving them away.
- It lets you use values without taking ownership
- It avoids cloning, which can be slow for large data
- It makes your programs safer and faster

## 9. Data structures

### 9.1. Arrays [*](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)
```rust
let array: [type; n] = [val1, val2, ..., val_n]
```
All values of the array must be of the same data type. We can access an element using index : `array[i]` for instance.

To change the value of one element in the array, the array must be mutable.

To access the length of the array, use `.len()` which is a class method of arrays

To loop through an array, just like in Python, you do : 
```rust
for elt in array {
    // code
}
```

To print a whole array :
```rust
let array = [val1, val2, ...]
println!("{:?}", array);
```

**Note** : The size of an array is fixed, no element can be deleted nor added. This is why we use Vectors sometimes which is a resizable array.

### 9.2. Vectors [*](https://doc.rust-lang.org/book/ch08-01-vectors.html)
```rust
let vector : Vec<type> = vec![val1, val2, ...]
```

Accessing and changing an element is the same as for arrays. Vectors also have a `.len()` method. The printing macro is also the same.

To add a new element in the vector, use `.push(value)`. To remove use `.pop()`. Their behavior is the same as `.append` `.pop()` in Python.

To add a new element at a certain index, use `.insert(index, value)`. To remove at a certain index, use `.remove(index)`. These two methods will change the whole vector and the complexity is linear in the number of element. Thus, it is not recommanded.

To loop through a vector, we must go through a reference of it (if not, the values are moved out and you can not longer use the vector) :
```rust
for elt in &vector {
    // code
}
```

A new vector can be initialized with two ways and the used method only depends on the developer preference : 
```rust
let vector = vec![];
let vector = Vec::new();
```

### 9.3. HashMap [*](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

To use HashMap, you must import it from Rust's standard library.
```rust
use std::collections::HashMap;

let mut hashmap = HashMap::new(); // To initialize a HashMap
hashmap.insert(key, value); // To insert a new pair (key, value)
hashmap.get(key); // To access a value from a key
hashmap.remove(key); // To remove a key and its associated value
println!("{:?}", hashmap); // To print the entire HashMap

for (key, value) in &hashmap { // To loop through a HashMap
    // code
}
```

If you insert a pair (key, value) and the key already exists in the HashMap, its value is updated.

### 9.4. Structs [*](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
```rust
struct Name {
    atr1 : type1,
    atr2 : type2,
    ...
}
```
To access an attribute, you follow the variable with a dot (.) and the attribute name.

Example : (we assume that a struct User was defined upper in the code with the correct attributes)
```rust
let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
```

### 9.5. Enums [*](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
```rust
enum Enumeraton {
    val1,
    val2,
    ...
    valn,
}
```

To use the enum, you have to use `::`. For example :
```rust
let variable = Enumeration::val1; 
```

Enums work well with match statement. As enums have a finite number of values, it is convenient to use it this way :
```rust
match my_enum {
    Enumeration::val1 => // code
    Enumeration::val2 => // code
    ...
    Enumeration::valn => // code
}
```

## 10. Summary

### Data structures
| Data structure     | Initialization                        | Common methods                                                                                    |
|---------------|---------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| **Vec\<T>**    | `let mut v = vec!new();`                | `.push(x)`, `.pop()`, `.len()`, `.iter()`, `.map()`, `v[i]`, `.get(i)`, `.contains()`, `.sort()`, `.clone()`, `.is_empty()`, `.resize(n, val)`, `.extend()`, `.retain()`, `.split_at()`, `.drain()`                                 |
| **[T; N]** (Array)   | `let a = [];`                        | `.len()`, `.iter()`, `a[i]`, `.map()`, `.copy_from_slice()`           |
| **HashMap\<K,V>**| `use str::collections::HashMap;` <br> `let mut m = HashMap::new();`             | `.insert(k, v)`, `.get(&k)`, `.remove(&k)`, `.contains_key(&k)`, `.keys()`, `.values()`, `.entry(k).or_insert(v)`, `.iter()`, `.len()`, `.clear()`, `.retain()`, `.clone()`                                                              |
| **Struct**    | `struct Point { x: f64, y: f64 }`           | `let p = Point { x: 1.0, y: 2.0 };` <br> Access : `p.x`, `p.y`                                              |
| **Tuple Struct** | `struct Color(u8, u8, u8);`              | Access with index : `c.0`, `c.1`, `c.2`                                                                                 |
| **Unit Struct** | `struct Empty;`                           | No fields — used to mark types or states                                                                   |
| **Enum**      | `enum Direction { Up, Down, Left, Right }` | Usage : `let d = Direction::Up;` <br> Match : `match d { Direction::Up => ... }`                            |
|               | Enum variants with or without data                        | Example : `enum Option<T> { Some(T), None }`                                               |

###  Usefull type conversions

| Conversion                         | Example                                                    |
|------------------------------------|-------------------------------------------------------------|
| Slice to Vec                       | `let v = s.to_vec();`                                       |
| Vec to slice                       | `let s: &[T] = &v[..];`                                     |
| Array to Vec                       | `let v = a.to_vec();`                                       |
| `Vec<f64>` to ndarray              | `Array1::from(vec)` (with `ndarray` crate)                  |
| Iterate over HashMap               | `for (k, v) in map.iter() { ... }`                          |
| Get mutable reference from struct  | `let p = &mut my_struct.field;`                             |

## Source
- [The Book (Rust official page)](https://doc.rust-lang.org/book/)
- [Rust tutorial (W3Schools)](https://www.w3schools.com/rust/rust_intro.php)"