// Given the recursive formula for Fibonacci sequence: F(n) = F(n-1) + F(n-2)
// Generate the nth number in the sequence.

use std::io;

fn main() {
    println!("Welcome to a Fibonacci sequence generator!");

    println!("Enter the nth number.");

    let mut nth_number = String::new();

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Failed to read line for nth number");

    let nth_number = nth_number.trim().parse::<i32>().unwrap();

    let fibonacci = (nth_number -1) + (nth_number - 2);

    println!("The {nth_number}th Fibonacci number is: {fibonacci}");
}
