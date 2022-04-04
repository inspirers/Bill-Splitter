use std::io;
use std::io::Write;

fn main() {
    print!("Enter amount for person 1: ");
    io::stdout().flush().unwrap();
    let mut input_line1 = String::new();
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line1) // actually read the line
        .expect("Failed to read line"); // which can fail, however
    let x: f32 = input_line1
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer"); // which, again, can fail

    print!("Enter amount for person 2: ");
    io::stdout().flush().unwrap();
    let mut input_line2 = String::new();
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line2) // actually read the line
        .expect("Failed to read line"); // which can fail, however
    let y: f32 = input_line2
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer"); // which, again, can fail

    if x > y {
        println!("Person 2 should pay Person 1: {}", ((x - y)/2.0));

    }
    else if y > x {
        println!("Person 1 should pay Person 2: {}", ((y - x)/2.0));
    }
    else if y == x {
        println!("No transfer should be made.");
    }
        //println!("{} {}", x, y);
}
