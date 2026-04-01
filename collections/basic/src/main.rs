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
    println!();
    println!("{:-^50}", "Accessing Elements at VEC");
    println!();

    let v5 = vec![2, 3, 5];
    let f1 = v5[0];
    println!("F1: {f1:?}");
    println!("V5: {v5:?}");
    let f4 = &v5[2];
    println!("F4: {f4:?}");

    // Accessing by get method ///
    let g1 = v5.get(0);
    println!("{g1:?}");

    let g2 = v5.get(4);
    println!("{g2:?}");

    let g = [2, 4, 5];
    println!("G:{:?}", g.get(2));

    println!();
    println!("{:-^50}", "Iteration on Vec");
    println!();

    // By For loop at Immutable referecen

    let i1 = vec![2, 4, 5];
    for ele in i1 {
        println!("Ele: {ele:?}");
    }

    // println!("I1 : {i1:?}"); //borrow of moved value, its ownership is just goes to i1, so here it can't accesss it

    // For accessing it after for looping...
    let mut i2 = Vec::new();
    i2.push(2);
    i2.push(3);
    i2.push(4);

    println!("I2: {i2:?}");

    for item in &i2 {
        // i2.iter() or &i2 -> both are reference to the same things
        // *item += 1;
        println!("Item: {item:?}");
    }

    println!("I2: {i2:?}");

    println!();
    println!("{:-^50}", "Multiple Types at Vector");
    println!();

    #[derive(Debug)]
    enum Spreadsheet {
        Text(String),
        Int(i32),
        Float(f32),
        Bool(bool),
    };

    let s1 = Spreadsheet::Text("Nishant".to_string());
    let s2 = Spreadsheet::Bool(false);
    let s3 = Spreadsheet::Int(-23);
    let s4 = Spreadsheet::Float(0.3);

    let mut mt = vec![];

    println!("{mt:?}");

    mt.push(s1);
    mt.push(s2);
    mt.push(s3);
    mt.push(s4);

    println!("Multiple types at vector: {mt:?}");

    for ele in &mt {
        // println!("ELE: {ele:?}");

        match ele {
            Spreadsheet::Bool(ele) => println!("{}", ele),
            Spreadsheet::Float(ele) => println!("{}", ele),
            Spreadsheet::Int(ele) => println!("{}", ele),
            Spreadsheet::Text(ele) => println!("{}", ele),
            _ => println!("Unknown"),
        }
    }
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
 - By index eg. v[0]
 - By get e.g   v.get(index)

 get method : It provides two things - Some or None
 By index: It could be panic

4. Iteration:
 - iter() and &a -> both are same to accessing element, just to give reference
 - iter_mut() and &mut a -> both are similar but it mutate the data, in-memory

5. Multiple Types:
 - For adding multiple types inside one single vector, we can create `enum` and their we can give multiple types

*/
