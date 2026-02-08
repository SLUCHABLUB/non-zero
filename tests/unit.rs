use non_zero::non_zero;

/// Test different kinds of expressions.
#[test]
fn expressions() {
    #![expect(clippy::unnecessary_cast, clippy::unnecessary_literal_unwrap)]

    const SIX: i64 = 6;

    // just a literal
    assert_eq!(non_zero!(2).get(), 2);
    // literal with a suffix
    assert_eq!(non_zero!(3u8).get(), 3);
    // literal with a spaced suffix
    assert_eq!(non_zero!(4_u64).get(), 4);
    // literal with a cast
    assert_eq!(non_zero!(5 as i16).get(), 5);
    // integer arithmetic
    assert_eq!(non_zero!(1 + 1).get(), 2_i32);
    // const method
    assert_eq!(non_zero!(Some(7).unwrap()).get(), 7);
    // constant
    assert_eq!(non_zero!(SIX).get(), 6);
}

/// Tests that the fallback type is the same as for normal integers.
#[test]
fn fallback_type() {
    use std::any::Any;
    assert_eq!(non_zero!(42).get().type_id(), 42.type_id());
}

/// Test edge cases.
#[test]
fn edges() {
    macro_rules! test_unsigned {
        ($ty:ty) => {
            assert_eq!(non_zero!(1 as $ty).get(), 1);
            assert_eq!(non_zero!(<$ty>::MAX).get(), <$ty>::MAX);
        };
    }

    macro_rules! test_signed {
        ($ty:ty) => {
            test_unsigned!($ty);
            assert_eq!(non_zero!(-1 as $ty).get(), -1);
            assert_eq!(non_zero!(<$ty>::MIN).get(), <$ty>::MIN);
        };
    }

    test_unsigned!(u8);
    test_unsigned!(u16);
    test_unsigned!(u32);
    test_unsigned!(u64);
    test_unsigned!(u128);
    test_unsigned!(usize);

    test_signed!(i8);
    test_signed!(i16);
    test_signed!(i32);
    test_signed!(i64);
    test_signed!(i128);
    test_signed!(isize);
}

/// Make sure that the result is `const`;
#[test]
fn constness() {
    use std::num::NonZero;
    const _: NonZero<i8> = non_zero!(5);
}
