use std::io;

fn main() {
    println!("guess the #!");
    println!("please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("u fucked up");

    println!("u guessed {}",guess);

}
