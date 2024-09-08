# mammon_newtype

Mammon is a lightweight, opinionated crate for working with money in Rust.

It provides a newtype wrapper around integer primitives that represent a number at three decimal places.

This is useful for working with currencies that have a fixed number of decimal places, such as the US dollar.

## Features

- Newtype wrapper around integer primitives for representing a number at three decimal places.
- Arithmetic operations between values.
- No standard library dependencies.

## Usage

```rust
let money = Milli64::new(12_300); // 12.30
let rate = Milli64::new(1_050); // 1.05
println!("{}", money * rate); // $12.915
```
