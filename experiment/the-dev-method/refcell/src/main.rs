use std::cell::{Ref, RefCell};

// fn main() {
//     // let x = RefCell::new(5); // x is NOT mutable

//     // *x.borrow_mut() += 1;

//     // println!("{}", x.borrow());
//     //
//     //
//     let g = RefCell::new(String::from("Nishant"));

//     *g.borrow_mut() = String::from("Amit");
//     println!("{}", g.borrow());

// }

fn main() {
    let x = RefCell::new(10);

    let a = x.borrow();
    let b = x.borrow_mut(); // ❌ runtime panic
}
