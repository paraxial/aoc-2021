use std::io;

// Poking away at https://doc.rust-lang.org/book/
fn main() {
    println!("Guess a number > ");

    // Variables are immutable by default: I presume that the `mut` means this isn't.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // read_line appends to a given string. As w/ c, & is a ref. Refs are also immutable.
        .expect("Failed to read line"); // Go read about io::Result.

    println!("weh: {}", guess);
}
