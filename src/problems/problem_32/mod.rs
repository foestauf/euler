use std::collections::HashSet;

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

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }

    let mut digits = [false; 9];

    for c in s.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        if digit == 0 || digits[digit - 1] {
            return false;
        }
        digits[digit - 1] = true;
    }

    true
}
