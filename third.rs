use std::io;

fn shortest_word(input: &str) -> Option<&str> {
    input.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let shortest = shortest_word(&input.trim());

    match shortest {
        Some(word) => println!("The shortest word is: {}", word),
        None => println!("No words found in the input."),
    }
}
