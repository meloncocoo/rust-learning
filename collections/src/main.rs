pub mod map;
pub mod result;
pub mod string;
pub mod vector;

fn main() {
    println!("Hello, world!");

    vector::demo();

    string::demo();
    string::chars();
    string::types();

    map::base();

    result::go();
}
