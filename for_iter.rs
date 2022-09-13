fn main() {
    let names = vec!["Bod", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("there is a rustacean among us"), // expected reference, not value

            _ => println!("Hello {}", name),
        }
    }

    println!("names {:?}", names);
}
