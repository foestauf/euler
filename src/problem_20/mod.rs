use num_bigint::BigUint;
use num_traits::{One};

pub fn solve() {
    let factorial = calculate_factorial(100);
    let sum_of_digits = sum_digits(&factorial);
    println!("The sum of the digits in the number 100! is: {}", sum_of_digits);
}

fn calculate_factorial(n: u32) -> BigUint {
    let mut factorial = BigUint::one();
    for i in 1..=n {
        factorial *= i;
    }
    factorial
}

fn sum_digits(n: &BigUint) -> u32 {
    n.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}
