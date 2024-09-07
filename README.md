# mammon

Rust library for ergonomically working with money as standard integer values.

## Features

- Newtype wrapper around all integer primtives for representing a number at two decimal places.
- Arithmetic operations between values.
- No standard library dependencies.

## Usage

```rust
let money = Centu32::new(1230); // 12.30
let rate = Centu32::new(5); // 0.05
println!("{}", money * money2); // $0.62
```

## Opionated Design

This library is meant to be small so it makes some assumptions.

- You are using a currency with two decimal places.
