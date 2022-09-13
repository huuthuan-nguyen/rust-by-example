fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // match a single value
        1 => println!("one"),
        // match several value
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        // match an inclusive range
        13..=19 => println!("a teen"),
        // handle the rest of cases, catch all
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // match is an expression too
    let binary = match boolean {
        // the arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
