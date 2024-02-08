use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    // Add remaining elements from arr1
    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    // Add remaining elements from arr2
    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

fn main() {
    println!("Enter sorted array 1 separated by spaces:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let arr1: Vec<i32> = input1.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter sorted array 2 separated by spaces:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let arr2: Vec<i32> = input2.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged sorted array: {:?}", merged);
}
