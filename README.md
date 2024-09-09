# mammon_newtype
Mammon is a lightweight, opinionated Rust crate designed for precise and efficient money calculations.
It offers a robust solution for handling financial operations with accuracy and performance in mind.

- Lightweight
- Uses standard integer types
- Arithmetic operations between values
- No standard library dependencies

## Why Mammon?

Mammon addresses a critical issue in financial software: the inherent imprecision of floating-point numbers. By using a fixed-point representation with three decimal places, Mammon ensures accurate calculations while avoiding the pitfalls of floating-point arithmetic.

## Why Mills

A [mill](https://en.wikipedia.org/wiki/Mill_(currency)) is 1/1000 of a base unit of currency– or in our case
any fixed-point number at three decimal places. Mammon includes several `Mill` types:

- `Millu64`: Mills newtype containing an unsigned 64-bit integer
- `Millu128`: Mills newtype containing an unsigned 128-bit integer
- `Milli64`: Mills newtype containing a signed 64-bit integer
- `Milli128`: Mills newtype containing a signed 128-bit integer

### Precision

Using 3 decimal places (representing thousandths of the base unit) provides greater precision than the traditional 2 decimal places (cents). This extra digit allows for:

- More accurate calculations, especially when dealing with fractional cents
- Better handling of small transactions or micro-payments
- Reduced cumulative rounding errors in large-scale operations

### Rounding control

- It's easier to truncate precision later if needed than to add it
- Accommodates potential changes in financial systems that may require more precision

## Usage

```rust
let money = Milli64::new(12_300); // 12.30
let rate = Milli64::new(1_050); // 1.05
println!("{}", money * rate); // $12.915
```

## FAQ

### Why not use `f32` or `f64`?

Floating-point numbers are imprecise and can lead to rounding errors. This is especially problematic in financial applications where accuracy is paramount.

### Why not use cents or 1/100 of a base unit of currency?

Using 3 decimal places provides greater precision than the traditional 2 decimal places. This extra digit allows for more accurate calculations, especially when dealing with fractional cents.

### Why no 32-bit support?

The max value will be `4,294,967.295`. Let's be real, even in the most
basic POS system, financial year totals may go over that. If you find the need to calculate higher numbers than that
in 32-bits, you might need to consider a different approach.

Similarly u16's max value is `65.535` and u8's max value is `0.255`. If you need these types, you can easily
create them by creating a new Mills type alias:

```rust
pub type Millu8 = Mills<u8>; // max of 25 cents
```

But really at this point, I'd recommend just doing the math in your head.

### Why no-std?

This library is designed to be as lightweight as possible. By avoiding the standard library, Mammon can be used in embedded systems, kernels, and other environments where the standard library is not available.

### Can I contribute?

Yes! Please open an issue or PR if you have any suggestions or improvements.

### Why mammon?

Mammon is a biblical term for money or love of money. It's a reminder that money is a tool, not an end in itself.

> Lay not up for yourselves treasures upon earth, where moth and rust doth corrupt, and where thieves break through and steal: But lay up for yourselves treasures in heaven, where neither moth nor rust doth corrupt, and where thieves do not break through nor steal: For where your treasure is, there will your heart be also. No man can serve two masters: for either he will hate the one, and love the other; or else he will hold to the one, and despise the other. Ye cannot serve God and mammon.
>
> — Matthew 6:19–21, 24 (KJV)

Regardless of what you think of religion, at least with this library, you can spend less time worrying about calculating the latter.
