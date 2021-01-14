use std::io;
use std::io::Write;

fn main() {
    print!("Please insert a number : ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("올바른 입력이 아닙니다.");

    println!("You've inserted the number : {}", guess);
}