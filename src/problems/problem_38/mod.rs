use crate::utils::is_pandigital;

pub fn solve() {
    let mut max_pandigital = 0;

    // Iterate over possible multiplicands.
    for multiplicand in 1..10_000 {
        let mut concatenated_product = String::new();
        let mut multiplier = 1;
        while concatenated_product.len() < 9 {
            concatenated_product.push_str(&(multiplicand * multiplier).to_string());
            multiplier += 1;
        }
        if concatenated_product.len() == 9 && is_pandigital(&concatenated_product) {
            max_pandigital = max_pandigital.max(concatenated_product.parse().unwrap());
        }
    }

    println!("The largest pandigital number is: {}", max_pandigital);
}

