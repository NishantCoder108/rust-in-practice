fn main() {
    let arr = [1, 2, 3, 3, 4, 4]; //store inside the stack instead of heap
    let mut vec: Vec<i32> = Vec::new();

    println!("{vec:?}");

    //for update

    vec.push(34);

    println!("Vec: {vec:?}");

    let mut v2 = vec![];

    println!("{v2:?}");

    v2.push(3);
    // let mut vec = vec![];

    // vec.push(4);
    // assert_eq!(4, vec[0]);
    // println!("Vec: {vec:?}");

    let mut v3 = vec![];

    println!("{v3:?}");

    let vect1 = VecT {
        id: 1,
        name: "Nishant".to_string(),
    };

    v3.push(vect1);

    println!("{v3:?}");

    let mut v4 = Vec::new();

    println!("{v4:?}");

    let e1 = ENUM::TECT;

    v4.push(e1);

    println!("{v4:?}");
}

#[derive(Debug)]
struct VecT {
    id: u32,
    name: String,
}

#[derive(Debug)]
enum ENUM {
    VEC,
    TECT,
}
/*
Collections:
1. Vectors - stores data that can increase at runtime
2. Hashmap - store as key value pairs
3. Strings - collections of characters


Vectors:

1. Creations: -
 - vec![]
 - Vec::new();

 //For creating variable or defining some value so for that we can use `vec![]` or `Vec::new()`


2. Updating: -
 - push(value)
 - required `mut` keyword


 Vec<T> :- T is types, so what types will be inside, here we can define
 e.g. Vec<i32> :- i32 types will be inside this vector. vector is nothing but it just store dynamic elements that can increase or decrease at runtime.

3. Accessing:-
 -


*/
