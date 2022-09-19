fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        // get  `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // match anything else (`None` variant).
        _ => (),
    }
}
