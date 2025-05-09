#![doc = include_str!("../README.md")]
#![no_std]

/// A macro for creating non-zero integers.
///
/// See the [crate level docs](crate).
#[macro_export]
macro_rules! non_zero {
    ($n:expr) => {
        const {
            // prettier errors
            if $n == 0 {
                panic!("tried initialising a non-zero value to zero")
            }
            ::core::num::NonZero::new($n).unwrap()
        }
    };
}
