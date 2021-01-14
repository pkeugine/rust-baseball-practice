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

    let mut user_numbers = Vec::new();
    for c in guess.trim().chars() {
        user_numbers.push(c.to_digit(10).unwrap());
    }
    // use & to borrow instead of moving ownership
    for number in &user_numbers {
        println!("{}", number);
    }
    println!("You've inserted the number : {}", guess);

    validate_user_numbers_size(user_numbers);
}

fn validate_user_numbers_size(user_numbers: Vec<u32>) {
    if user_numbers.len() != 3 {
        println!("amount of user numbers is not valid");
    } else {
        println!("amount of user numbers is valid!");
    }
}