use std::io;
use test_practice::cipher::{Cipher, self};




fn main() {
    // println!("Enter the text to encrypt:  ");
    // let mut my_text = String::new();

    // io::stdin().read_line(&mut my_text).expect("Cannot read input");
    // print!("Your encrypted text: {}", rot13::Rot13(my_text).encrypt());

    println!("Input the string to encrypt: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let encrypted_input = cipher::rsa::Rsa::new(
        user_input
    ).expect("Encryption failed!");
    let encrypted_string = encrypted_input.encrypted_string().expect("");
    println!("Encrypted string:\n {}", encrypted_string);

    let decrypted_string = encrypted_input.original_string().expect("");
    println!("Decrypted String:\n {}", decrypted_string);


}
