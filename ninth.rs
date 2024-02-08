use std::io;

fn main() {
    println!("Enter a string to reverse:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let reversed = reverse_string(&input.trim());

    println!("Reversed string: {}", reversed);
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}
