

**Dyna Language Documentation**

## Table of Contents
1. [Overview](#overview)
2. [Variables and Immutability](#variables-and-immutability)
3. [Control Flow and Pattern Matching](#control-flow-and-pattern-matching)
4. [Functions](#functions)
5. [Arrays](#arrays)
6. [Type System and Type Checking](#type-system-and-type-checking)
7. [Examples](#examples)


### 1. Overview <a name="overview"></a>
Dyna is a language inspired by Rust, focusing on immutability, strict typing, and pattern matching. This documentation outlines the syntax, core concepts, and usage patterns of Dyna, guiding developers through its functionality.

### 2. Variables and Immutability <a name="variables-and-immutability"></a>
In Dyna, variables are **immutable by default**. Variables are declared using the `let` keyword. To make a variable mutable, add the `mut` keyword.

#### Syntax
```rust
let variable_name = String::new();       // Immutable variable
let mut variable_mutable = String::from("Hello"); // Mutable variable
```

#### Note
The final expression in a block does not require a semicolon:
```rust
let variable = if true {
    String::new()
} else {
    String::from("false")
};
```

### 3. Control Flow and Pattern Matching <a name="control-flow-and-pattern-matching"></a>
Dyna includes **pattern matching** and supports **user-defined types** such as enumerations (`enum`) and structs.

#### Syntax
```rust
enum T {
    Some(String),
    None,
}

let var = T::None;

match var {
    T::Some(string) => println!(string),
    T::None => (),
}
```

### 4. Functions <a name="functions"></a>
Functions in Dyna are declared with the `fn` keyword. Each parameter requires an explicit type.

#### Syntax
```rust
fn name_of_function(mut arg: String) -> String {
    arg.push_str("value");
    arg
}
```

#### Return Types
Functions can specify a return type using `->`. The default return type is `()` (unit type).

### 5. Arrays <a name="arrays"></a>
Dyna supports compile-time arrays, requiring explicit types since type inference is not supported.

#### Syntax
```rust
let array: &[u8] = [1, 2, 3, 4];
```

### 6. Type System and Type Checking <a name="type-system-and-type-checking"></a>
Dyna performs **compile-time type checking** to prevent mismatched types. While Dyna does not yet support advanced type theories, concrete type matching ensures type consistency.

#### Example of Type Checking
```rust
fn name_of_function(mut arg: String) -> String {
    arg.push_str("value");
    arg
}

let array: &[u8] = [1, 2, 3, 4];
// Error: Expected `String`, found `&[u8]`
name_of_function(array);
```

### 7. Examples <a name="examples"></a>
Below are several examples demonstrating Dyna’s features.

#### Example 1: Immutable and Mutable Variables
```rust
let x = 5;       // Immutable
let mut y = 10;  // Mutable
y += x;
```

#### Example 2: Pattern Matching with Enums
```rust
enum Option {
    Some(i32),
    None,
}

let value = Option::Some(10);

match value {
    Option::Some(num) => println!("Value is: {}", num),
    Option::None => println!("No value"),
}
```

#### Example 3: Function with Type Error
```rust
fn add_prefix(mut text: String) -> String {
    text.push_str("prefix_");
    text
}

let arr: &[u8] = [1, 2, 3];
// Error: Expected `String`, found `&[u8]`
add_prefix(arr);
```

---


