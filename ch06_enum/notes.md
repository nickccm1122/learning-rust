# CH06 Enums

## Define an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

## Another Representation

We can attach data in each variant of enum directly.

```rust
enum IpAddr {
  V4(u32, u32, u32, u32),
  V6(String)
}

let home = IpAddr::V4(127, 0, 0, 1);
let office = IpAddr::V6(String::from("::1"));
```

### Examples

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

- `Quit` has no data associated with it at all.
- `Move` includes an anonymous struct inside it.
- `Write` includes a single String.
- `ChangeColor` includes three i32 values.

## `Option` Enum

see `main.rs`

## `match` control flow

see `main.rs`

## Using `if let` control flow

Think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
