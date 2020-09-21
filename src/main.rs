use std::io;

use fizzbuzz::count_up_to;
use fizzbuzz::parse_input;

fn main() {
    wellcome();
}


fn wellcome() {
    println!("Fizzbuzz, please enter a the number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = parse_input(number);
    
    count_up_to(number, &mut std::io::stdout());
}