struct A;

// concrete type `A`
struct S(A);

// concrete type `S`
struct SGen<T>(T); // generic type `SGen`

// the following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable.

// define a function `reg_fn` that takes an argument `_s` of typ `S`.
// this has no `<T>` so this is not a generic function.
fn reg_fn(_s: S) {}

// define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// it has been explicitly given the type parameter `A`, but because `A` has not
// been specified as a generic type parameter of `gen_spec_t`, it is no generic.
fn gen_spec_t(_s: SGen<A>) {}

// define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`
// it has been explicitly given the type parameter `i32`, which is a specific type.
// because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// because `SGen<T>` is preceded by `T`, this function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // using the non-generic functions.
    reg_fn(S(A)); // concrete type
    gen_spec_t(SGen(A)); // implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // implicitly specified type parameter `i32`.

    // explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}