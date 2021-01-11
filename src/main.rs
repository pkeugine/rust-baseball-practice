use std::io;
use std::io::Write;

fn main() {
    print!("숫자를 입력해주세요 : ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("숫자를 입력받지 못했습니다.");

    println!("당신이 입력한 숫자입니다 : {}", guess);
}
