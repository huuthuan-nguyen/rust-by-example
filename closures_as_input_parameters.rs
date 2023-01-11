// a function which takes a closure as an argument and calls it
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
where
    // the closure takes no input and returns nothing
    F: FnOnce(),
{
    // ^ TODO: Try changing this to `Fn` or `FnMut`

    f()
}

// a function which takes a closure and returns an `i32`
fn apply_to_3<F>(f: F) -> i32
where
    // the closure takes an `i32` and returns an `i32`
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // a non-copy type
    // `to_owned` creates owned data form borrowed one
    let mut farewell = "goodbye".to_owned();

    // capture 2 variables: `greeting` by reference and
    // `farewell` by value
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now required `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
