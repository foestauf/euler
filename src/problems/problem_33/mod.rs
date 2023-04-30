

pub fn solve() {
    let mut product_numerator: u64 = 1;
    let mut product_denominator: u64 = 1;

    for numerator in 10..100 {
        for denominator in (numerator + 1)..100 {
            if is_curious_fraction(numerator, denominator) {
                let gcd = num::integer::gcd(numerator, denominator);
                let reduced_numerator = numerator / gcd;
                let reduced_denominator = denominator / gcd;

                product_numerator *= reduced_numerator as u64;
                product_denominator *= reduced_denominator as u64;

                let product_gcd = num::integer::gcd(product_numerator, product_denominator);
                product_numerator /= product_gcd;
                product_denominator /= product_gcd;
            }
        }
    }

    println!("The denominator of the simplified product is: {}", product_denominator);
}



fn is_curious_fraction(numerator: u32, denominator: u32) -> bool {
    let numerator_digits = to_digits(numerator);
    let denominator_digits = to_digits(denominator);

    for &num_digit in &numerator_digits {
        if num_digit != 0 && denominator_digits.contains(&num_digit) {
            let cancelled_numerator = remove_digit(&numerator_digits, num_digit);
            let cancelled_denominator = remove_digit(&denominator_digits, num_digit);

            if cancelled_denominator == 0 {
                continue;
            }

            if cancelled_numerator as f64 / cancelled_denominator as f64 == numerator as f64 / denominator as f64 {
                return true;
            }
        }
    }

    false
}


fn to_digits(number: u32) -> Vec<u32> {
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn remove_digit(digits: &Vec<u32>, digit: u32) -> u32 {
    let mut new_digits = digits.clone();
    if let Some(index) = new_digits.iter().position(|&d| d == digit) {
        new_digits.remove(index);
    }

    new_digits.into_iter().fold(0, |acc, d| acc * 10 + d)
}
