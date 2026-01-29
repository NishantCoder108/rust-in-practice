fn main() {
    let boxed = Box::new(5);
    let vector = vec![9, 8, 7];

    println!("&boxed (stack) = {:p}", &boxed);
    println!("boxed.ptr (heap) = {:p}", Box::into_raw(boxed)); // consumes boxed
    // recreate boxed to avoid use-after-free in this toy example:
    // let boxed = unsafe { Box::from_raw(boxed_ptr) };

    println!("&vector (stack) = {:p}", &vector);
    println!("vector.as_ptr() (heap) = {:p}", vector.as_ptr());

    // keep vector alive so heap pointer remains valid
    std::mem::forget(vector);
}
