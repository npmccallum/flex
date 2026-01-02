# flex

[![CI](https://github.com/npmccallum/flex/workflows/Test/badge.svg)](https://github.com/npmccallum/flex/actions)
[![Crates.io](https://img.shields.io/crates/v/flex.svg)](https://crates.io/crates/flex)
[![Documentation](https://docs.rs/flex/badge.svg)](https://docs.rs/flex)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Flexible borrowing and ownership for Rust.

`Flex` is an enum that holds either a borrowed reference or an owned boxed value
of the same type. It is similar in concept to `Cow` and works seamlessly with
unsized types like `dyn Trait`, `[T]`, and `str`.

## Installation

```toml
[dependencies]
flex = "0.1"
```

If your crate has an `alloc` feature, pass it through to `flex`:

```toml
[features]
alloc = ["flex/alloc"]

[dependencies]
flex = "0.1"
```

## Quick Start

```rust
use flex::Flex;

// Start with a borrowed slice
let borrowed = Flex::Lend(&[1, 2, 3][..]);
assert_eq!(&*borrowed, &[1, 2, 3]);

# #[cfg(feature = "alloc")] {
// Or own a slice
let owned = Flex::Give(vec![4, 5, 6].into_boxed_slice());
assert_eq!(&*owned, &[4, 5, 6]);

// Convert borrowed to owned
let claimed: Flex<'static, [i32]> = borrowed.claim();
# }
```

## Flex vs Cow

While both `Flex` and `Cow` deal with borrowed vs owned data, they serve
different purposes:

### Cow (Clone-on-Write)

- Works with **type pairs** (e.g., `&str`/`String`, `&[T]`/`Vec<T>`)
- Uses `ToOwned` trait for conversion
- Requires `alloc` - not available in `no_std`

### Flex (Flexible Ownership)

- Works with **ownership models** of the same type (`&T` vs `Box<T>`)
- Works seamlessly with like **unsized types** `dyn Trait`, `[u8]` and `str`
- No `ToOwned` requirement
- Works in `no_std` without `alloc` - **produces consistent APIs**

Example: `Flex<'a, str>` holds either `&'a str` or `Box<str>`, while
`Cow<'a, str>` holds either `&'a str` or `String`.

## Use Cases

`Flex` is particularly useful when:

- Working with trait objects: `Flex<'a, dyn Debug>` holds `&dyn Debug` or
  `Box<dyn Debug>`
- Building APIs that accept both borrowed and owned unsized types
- Flexible ownership without `ToOwned` constraints
- Deferring allocation decisions until runtime

## Features

- `alloc`: Enables the `Give` variant with `Box<T>`

Without `alloc`, `Flex` only supports the `Lend` variant, but APIs remain
compatible.

## License

Licensed under the [MIT License](LICENSE).
