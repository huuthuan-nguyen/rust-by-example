fn main() {
    crates_creating_library::public_function();

    // Error! `private_function` is private
    // crates_creating_library::private_function();

    crates_creating_library::indirect_access();
}
