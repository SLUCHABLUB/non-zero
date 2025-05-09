//! Inverse ui test.
//!
//! Due to a bug in the `trybuild` crate,
//! `panic`s in `const` blocks do not cause compile-errors when run with `compile_fail`,
//! but they do when run with `pass`.
//!
//! As a hacky and temporary fix, we therefore run `pass` but with `should_panic`.
//!
//! Care should be taken since the error messages are not compared like with `compile_fail`.

use trybuild::TestCases;

#[test]
#[should_panic]
fn ui_should_panic() {
    let test_cases = TestCases::new();
    test_cases.pass("tests/ui-no-pass/*.rs");
}
