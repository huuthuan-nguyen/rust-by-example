use std::marker::PhantomData;

// a phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // allow equality test for this type.
struct PhantomTuple<A, B> (A, PhantomData<B>);

// a phantom type struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // allow equality test for this type.
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// note: storage is allocated for generic type `A`, but not for `B`.
// therefore, `B` cannot be used in computations.

fn main() {
    // here, `f32` and `f64` are the hidden parameters.
    // PhantomTuple type specified as `<char, f32>`.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);

    // PhantomTuple type specified as `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // type specified as `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // type specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // println!("_tuple1 == _tuple2 yield: {}", _tuple1 == _tuple2);
    //
    // println!("_struct1 == _struct2 yield: {}", _struct1 == _struct2);
}