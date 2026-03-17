use crate::List::{Cons, Nil};
use std::cell::RefCell;

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

// fn main() {
//     let x = RefCell::new(10);

//     let a = x.borrow();
//     let b = x.borrow_mut(); // ❌ runtime panic
// }
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
