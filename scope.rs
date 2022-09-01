fn main() {
    // this binding lives in the main function
    let long_lived_binding = 1;

    // this is a block, and has a smaller scope than the main function
    {
        // this binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    // end of the block

    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}
