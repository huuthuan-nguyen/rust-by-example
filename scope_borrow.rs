// this function take ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// this function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("this int is: {}", borrowed_i32);
}

fn main() {
    // create a boxed i32, and a stacked i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // error!
        // can't destroy `boxed_i32` while the inner value is borrowed later in scope
        // eat_box_i32(boxed_i32);

        // attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}
