use hey_nishant::HeyNishant;
use my_macro::Hello;

#[derive(Hello, HeyNishant)]
struct User;

fn main() {
    User::hey();
}
