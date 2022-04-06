use std::io;
use std::io::Write;

fn input(n: u8) -> f32 {
    print!("Enter amount for person {}: ", n);

    io::stdout().flush().unwrap();
    let mut input_line = String::new();
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however

    let x: f32 = input_line
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Failed to parse from string to float");
    return x;
}

fn calc(x: f32, y: f32) {
    if x > y {
        println!("Person 2 should pay person 1: {}", ((x - y) / 2.0));
    } else if y > x {
        println!("Person 1 should pay person 2: {}", ((y - x) / 2.0));
    } else if y == x {
        println!("No transfer should be made.");
    }
}

fn main() {
    let x: f32 = input(1);
    let y: f32 = input(2);
    calc(x, y);
}
