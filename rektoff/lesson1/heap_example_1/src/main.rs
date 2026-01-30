fn main() {
    let v = vec![(44, "a"), (55, "b")]; /*Type of v = Vec<(i32, &str)> */

    println!("Vec ptr (heap start): {:p}", v.as_ptr());//Vec ptr (heap start): 0x104c7d980
    println!("v[0] value: {:?}", v[0]);
    println!("v[1] value: {:?}", v[1]);

    /* Pointer to the whole tuple */
    let p = &v[0] as *const (i32, &str);//q -> v[1] tuple ptr: 0x104c7d998
     //pointing to same v variable pointer and it will contagious increase
     // *const -> read only pointer , *mut -> mutable pointer
    println!("p -> v[0] tuple ptr: {:p}", p);

    let q = &v[1] as *const (i32, &str); //pointing to same v variable pointer and it will contagious increase
    println!("q -> v[1] tuple ptr: {:p}", q); //q -> v[1] tuple ptr: 0x104c7d998
    /* It means pointer of v = p pointer */


    /*-------------Nested Inside----------- */

    let g = v[0];
    let h = v[1];  
    println!("v : {:?}", g.1.as_ptr());//0x10258dc93
    println!("h : {:?}", h.1.as_ptr());//0x10258dc94

    let p_i32 = &v[0].0 as *const i32;
    println!("v[0].0 ptr: {:p}", p_i32); //v[0].0 ptr: 0x102c45960

    let p_str = &v[0].1 as *const &str;
    println!("v[0].1 (&str) ptr: {:p}", p_str); //v[0].1 (&str) ptr: 0x102c45968

    let p_bytes = v[0].1.as_ptr();
    println!("\"a\" bytes ptr: {:p}", p_bytes);//"a" bytes ptr: 0x10258dc93



}
