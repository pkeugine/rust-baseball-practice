use rand::Rng;
use std::collections::HashSet;
use std::io;
use std::io::Write;

fn main() {
    // generate computer numbers without duplicate numbers
    let mut computer_numbers = HashSet::new();
    while computer_numbers.len() < 3 {
        computer_numbers.insert(rand::thread_rng().gen_range(1..10));
    }
    for number in &computer_numbers {
        println!("{}", number);
    }

    // get user input
    print!("Please insert a number : ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Invalid Input");

    println!("You've inserted the number : {}", guess);
}
