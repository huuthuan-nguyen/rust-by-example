// this enum purposely neither implements nor derives PartialEq.
// that is why comparing Foo::Bar == a fails below.
enum Foo {
    Bar,
}

fn main() {
    let a = Foo::Bar;

    // variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
