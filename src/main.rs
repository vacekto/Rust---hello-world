use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please enter your message:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("your message to the world is: {}", guess)
}
