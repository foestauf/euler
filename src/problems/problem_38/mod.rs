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

fn is_pandigital(number: &str) -> bool {
    if number.len() != 9 {
        return false;
    }
    let mut digits = [false; 9];
    for ch in number.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        if digit == 0 || digits[digit - 1] {
            return false;
        }
        digits[digit - 1] = true;
    }
    true
}
