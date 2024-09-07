# mammon
Rust lib crate for ergonomically working with money values.

Currently work in progress.

## Usage

```rust
let money = Money::from("$2.50".to_string());
let money2 = Money::from("$20.05".to_string());
println!("{}", money * money2); // $50.12
```
