// fn main() {
//     panic!("crash and burn");
// }
// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }

struct CleanupDetector;

impl Drop for CleanupDetector {
    fn drop(&mut self) {
        println!("✨ Cleanup: The destructor was called!");
    }
}

fn main() {
    let _detector = CleanupDetector;
    println!("About to panic...");
    panic!("Normal panic occurs!");
}
