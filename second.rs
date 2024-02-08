use std::io;

fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result: Option<usize> = None;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; // Look for the first occurrence on the left side
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    println!("Enter sorted array of integers separated by spaces:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the target number:");
    
    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input).expect("Failed to read line");
    
    let target: i32 = target_input.trim().parse().expect("Invalid input");

    if let Some(index) = first_occurrence_index(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not found in the array.", target);
    }
}
