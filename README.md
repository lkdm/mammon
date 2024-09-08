# mammon

Rust library for ergonomically working with money as standard integer values.

## Features

- Newtype wrapper around all integer primtives for representing a number at three decimal places.
- Arithmetic operations between values.
- No standard library dependencies.

## Usage

```rust
let money = Milli64::new(12_300); // 12.30
let rate = Milli64::new(1_050); // 1.05
println!("{}", money * rate); // $12.915
```
