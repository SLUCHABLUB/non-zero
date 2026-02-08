# non-zero

A macro for creating constant non-zero integers (with type inference).

There are multiple crates that address the issue of initialising non-zero integers.
However, most of them lack type inference.
This is why I created `non_zero!`.
It uses `const` blocks to evaluate the literals at compile time whilst achieving type inference.

The definition is essentially this:

```rust
macro_rules! non_zero {
    ($n:expr) => {
        const {
            NonZero::new($n).unwrap()
        }
    };
}
```

Some things of note:

- `$n:expr` allows for any expression to be passed to the macro, so long as it is evaluable in a `const` context.
- The `const` block is discussed above.
- `NonZero` is the `std` generic non-zero type to which all `NonZero***` are aliased.
  The generic argument can be inferred, not only from `$n` but also from the macro's usage.
- `.unwrap()` inside the `const` block will cause a compile-time error, not a runtime one.

The above implementation is somewhat simplified;
the real definition produces prettier errors than `unwrap` and is more hygienic.

## Naming

This crate uses the same name as `std` does for its types.
Namely, with a hyphen between "non" and "zero".
In `snake_case` the hyphen becomes an underscore and in `PascalCase` it creates a word break.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
