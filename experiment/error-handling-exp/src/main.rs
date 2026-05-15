use std::{fs::File, io::ErrorKind, panic};
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// fn main() {
//     let greeting = File::open("hello.txt");

//     println!("File Greeting: {:?}", greeting);

//     let greeting_file = match greeting {
//         Ok(file) => file,
//         Err(err) => match err.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(file) => file,
//                 Err(err) => panic!("Problem with creating the file : {:?}", err),
//             },

//             _ => panic!("Problem with opening the file: {:?}", err),
//         },
//     };

//     println!("Greeting file : {:?} ", greeting_file);
// }

fn main() {
    let greeting = File::open("world.txt");

    let g_file = greeting.unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|err| {
                panic! {"problem with creating file: {err:?}"}
            })
        } else {
            panic!("Problem with opening the file: {err:?}")
        }
    });

    println!("Greetings file : {g_file:?}");
}
