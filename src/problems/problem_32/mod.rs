use std::collections::HashSet;
use crate::utils::is_pandigital;

pub fn solve() {
    let mut products = HashSet::new();

    for a in 1..100 {
        for b in 1..10000 {
            let product = a * b;
            let concatenated = format!("{}{}{}", a, b, product);
            if concatenated.len() == 9 && is_pandigital(&concatenated) {
                products.insert(product);
            }
        }
    }

    let sum: u32 = products.iter().sum();
    println!("The sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital: {}", sum);
}

