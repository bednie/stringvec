use stringvec::stringvec;
use std::f64::consts::PI;

fn main() {
    let mixed_types = stringvec!["hello", 42, 'A', PI];
    println!("Mixed types vector: {:?}", mixed_types);

    let names = stringvec!["Alice", "Bob", "Charlie"];
    println!("Names: {:?}", names);

    // Using with variables
    let age = 30;
    let height = 5.9;
    let info = stringvec!["John Doe", age, height];
    println!("Person info: {:?}", info);
}