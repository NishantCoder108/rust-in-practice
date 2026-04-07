use std::collections::HashMap;

fn main() {
    let coll: HashMap<&str, i32> =
        HashMap::from([("Communication Skills", 4), ("Rust", 1), ("TS", 2)]);
    println!("Coll: {coll:?}");

    let mut vec_coll: Vec<_> = coll.into_iter().collect();

    vec_coll.sort_by_key(|k| k.1);

    println!("{vec_coll:?}");

    let res: Vec<_> = vec_coll.iter().map(|k| k.0).collect();

    println!("Res: {res:?}");

    print!("{vec_coll:?}");

    // for (key, value) in coll.iter() {
    //     println!("Key: {key:?}, Value : {value:?}");

    // }
}
