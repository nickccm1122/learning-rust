# CH07 Packages, Crates and Modules

- Packages are a Cargo feature that let you build, test, and share crates.
- Crates are a tree of modules that produce a library or executable.
- Modules and the use keyword let you control the scope and privacy of paths.
- A path is a way of naming an item such as a struct, function, or module.

## Package Convention

Cargo’s conventions are that if you have a `src` directory containing `main.rs` in the same directory as a package’s `Cargo.toml`, Cargo knows this package contains a binary crate with the same name as the package, and `src/main.rs` is its crate root.

Another convention of Cargo’s is that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root.

## Module System

- modules are the privacy boundary in Rust -> they are private by default
- use the `pub` keyword to make them public
- use `super` to access parent module
- If you use pub before a struct definition, you make the struct public. However, the struct’s fields are still private.
- if you make a public enum, all of its variants are public.
- `use` keyword brings item into scope
  - `use crate::**` for absolute path
  - `use self::**` for relative path <- try not to use as people are working on eliminating.

## Idiomatic `use`

- It’s considered idiomatic to specify the function’s parent module with use, and then specify the parent module when calling the function.
- For structs, enums, and other items, specifying the full path to the item with use is idiomatic.

## `as` keyword

- `use std::io::Result as IoResult;`

## Re-exporting

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}
```

## Optimize vertical `use` space

from

```rust
use std::cmp::Ordering;
use std::io;
```

to

```rust
use std::{cmp::Ordering, io};
```


### Bring all public exports into scope

```rust
use std::collections::*;
```