// fn main() {
//     // println!("Hello, world!");
//     let v: Vec<u32> = vec![1, 2, 3];
// }
/* ------ Declarative Macros ------- */
// macro_rules! say_hello {
//     () => {
//         println!("Hello!");
//     };
// }

// fn main() {
//     say_hello!();
// }

macro_rules! my_vec {
    ( $( $x:expr );* ) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}

fn main() {
    let v = my_vec![1; 2; 3];
    println!("{:?}", v);
}

// macro_rules! hi_macro {
//     () => {
//         println!("Hi");
//     };
//     ($msg:expr) => {
//         println!("Hi, {}", $msg);
//     };
// }

// fn main() {
//     hi_macro!();
//     hi_macro!("Nishant");
// }
