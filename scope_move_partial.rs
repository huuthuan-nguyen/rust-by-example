fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced.
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);

    println!("The person's age from person struct is {}", person.age);
}
