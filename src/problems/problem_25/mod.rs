pub fn solve() {
    let num_digits = 1000;
    let index = find_fibonacci_index_with_digits(num_digits);

    println!("The index of the first term in the Fibonacci sequence to contain {} digits is: {}", num_digits, index);
}

use num_bigint::{BigUint};
use num_traits::{One};

fn find_fibonacci_index_with_digits(num_digits: usize) -> usize {
    let mut index = 2;
    let mut prev = BigUint::one();
    let mut current = BigUint::one();
    // let threshold = 10usize.pow((num_digits - 1) as u32);

    while current.to_string().len() < num_digits {
        let next = &prev + &current;
        prev = current;
        current = next;
        index += 1;
    }

    index
}

