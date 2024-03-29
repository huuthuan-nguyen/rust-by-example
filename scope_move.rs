fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // both values can be dependently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;

    // the pointer address of `a` is copied (not the data) into `b`.
    // both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    // println!("a contains: {}", a);

    // this function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // error! same reason as the previous error
    // println!("b contains: {}", b);
}