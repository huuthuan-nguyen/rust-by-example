fn main() {
    let mut _mutable_integer = 7i32;

    {
        // shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
    }

    // OK! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
