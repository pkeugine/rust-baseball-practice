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
    validate(&*guess);

    let mut user_numbers = Vec::new();
    for c in guess.trim().chars() {
        user_numbers.push(c.to_digit(10).unwrap());
    }
    // use & to borrow instead of moving ownership
    for number in &user_numbers {
        println!("{}", number);
    }
    println!("You've inserted the number : {}", guess);
}

fn validate(guess: &str) {
    let mut temp_numbers = Vec::new();
    for c in guess.trim().chars() {
        temp_numbers.push(c);
    }
    validate_length(&*temp_numbers);
    validate_integer(&*temp_numbers);
    validate_positive(&*temp_numbers);
}

fn validate_length(temp_numbers: &[char]) {
    if temp_numbers.len() != 3 {
        println!("amount of user numbers is not valid");
    }
}

fn validate_integer(temp_numbers: &[char]) {
    for c in temp_numbers {
        if !c.is_digit(10) {
            println!("non-digit input has been found");
        }
    }
}

fn validate_positive(temp_numbers: &[char]) {
    for c in temp_numbers {
        if c.to_digit(10).unwrap() <= 0 {
            println!("non-positive input has been found");
        }
    }
}

