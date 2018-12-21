# CH05 Defining Struct

```rust
// to declare a struct
struct User {
  username: String,
  email: String,
  sing_in_count: u64,
  active: bool,
}

// to create a struct
let user1 = User {
  user: String::from("Ming Chan"),
  email: String::fron("nickccm@pm.me"),
  sing_in_count: 1,
  action: true,
};

// struct update syntax.
// with struct update syntax.
let user2 = User {
  user: String::from("Index"),
  email: String::fron("index@pm.me"),
  sing_in_count: 1,
  action: true,
}

// with struct update syntax.
let user3 = User {
  user: String::from("Index"),
  email: String::fron("index@pm.me"),
  ..user1
}
```

> '..' specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

## Tuple struct

Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields;

```rust
struct Color (i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let point = Point(1, 2, 3);
```

## Ownership of a Struct

```rust
let user1 = User {
  user: String::from("Ming Chan"),
  email: String::fron("nickccm@pm.me"),
  sing_in_count: 1,
  action: true,
};
```

The struct here has all the ownership of its field values.

## Adding Additional Functionality with Derived Trait

For primitive types, there are already a Display function implemented, for struct, however, has not.

use `:?` tells `println!` that we want to use a `Debug` Formatting

make sure to annotate the struct to allow debug formatting

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Also, can use {:#?} in println!("{}", )
```

## Methods

Methods are very similar to function, however, they are defined within a context of a struct.

1. Their first param is always `self`

## Associate Function

For method implemented in a struct without `&self` as first param, it is associate function.

```rust
// we use an associate function in this way, its like a static function
Rectangle::square()
```