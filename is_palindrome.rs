use std::io;

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();
    let reversed = input.chars().rev().collect::<String>();
    input == reversed
}

fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let input = input.trim(); // Remove trailing newline
    
    if is_palindrome(input) {
        println!("'{}' is a palindrome!", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
