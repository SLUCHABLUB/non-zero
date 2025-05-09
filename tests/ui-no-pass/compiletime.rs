use std::hint::black_box;
use non_zero::non_zero;

fn main() {
    if black_box(false) {
        non_zero!(0);
    }
}
