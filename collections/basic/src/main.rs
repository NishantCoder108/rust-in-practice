fn main() {
    let mut vec: Vec<i32> = Vec::new();

    println!("{vec:?}");

    //for update

    vec.push(34);

    println!("Vec: {vec:?}");

    // let mut vec = vec![];

    // vec.push(4);
    // assert_eq!(4, vec[0]);
    // println!("Vec: {vec:?}");
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

*/
