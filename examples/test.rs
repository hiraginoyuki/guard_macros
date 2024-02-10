use guard_macros::{guard, make_guard};

fn main() {
    make_guard!(guard_panic => panic!("hello"));

    guard! {
        (a, b) = (1, 2),
        Some(c) = Some(3),
        a <= b,
        a + b == c,
    }

    guard_panic!(false);

    println!("hello, a = {a}")
}