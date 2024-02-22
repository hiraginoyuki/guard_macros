use guard_macros::make_guard;

#[cfg_attr(test, test)]
fn main() {
    make_guard!(guard => _);

    guard!((a, b) = (1, 2));
    assert!(a != b);

    guard! {
        // {
        //     foo = 1,
        // } => _,
        *{
            bar = 2,
        } => _,
    }

    // doesn't compile
    // println!("{foo}");
    println!("{bar}");
}
