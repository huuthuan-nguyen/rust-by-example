// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// a function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // error! `_x` does not live long enough
    // let y: &'a i32 = &_x;
    // attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime cannot be coerced into longer one.
}

fn main() {
    // create variables to be borrowed below.
    let (four, nine) = (4, 9);

    // borrow (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // any input which is borrowed must outlive the borrower.
    // in other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // because the lifetime is never constrained, it defaults to `'static`.
}
