use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;

    for &num in arr {
        current_sum = current_sum + num;
        if current_sum < 0 {
            current_sum = 0;
        }
        if max_sum < current_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}

fn main() {
    println!("Enter the array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
