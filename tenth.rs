use std::io;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: u64 = input.trim().parse().expect("Invalid input");

    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}
