enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let x = Operations::Add;
    let y = Operations::Subtract;
    println!("Add 1 + 2 = {}", x(1, 2));
    println!("Subtract 4 - 2 = {}", y(4, 2));
}
