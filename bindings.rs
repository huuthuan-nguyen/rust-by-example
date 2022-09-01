fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copier_integer = an_integer;

    println!("And integer: {:?}", copier_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // the compiler warns about unused variable bindings; these warning can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;
}
