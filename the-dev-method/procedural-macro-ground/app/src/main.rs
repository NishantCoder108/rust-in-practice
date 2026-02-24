use my_macro::Hello;

#[derive(Hello)]
struct User;

fn main() {
    println!("Hello, world!");
    User::hello();
}
