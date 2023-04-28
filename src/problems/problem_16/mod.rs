use num_bigint::BigUint;
use num_traits::pow;

pub fn solve() {
    let base: BigUint = 2u32.into();
    let exponent = 1000;
    let number = pow(base, exponent);

    let digit_sum: u32 = number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    println!("The sum of the digits of 2^1000 is: {}", digit_sum);
}
