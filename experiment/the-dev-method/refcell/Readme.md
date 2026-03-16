## RefCell
- It can mutate data without using `mut` keyword. It happen at runtime level. 

- Many borrow allowed but only one mutable borrow is allowed at a time.

eg.
```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(10);

    let a = x.borrow();
    let b = x.borrow_mut(); // ❌ runtime panic
}
```
