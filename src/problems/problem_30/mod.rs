pub fn solve() {
    let sum = find_sum_of_fifth_power_digit_numbers();
    println!(
        "The sum of all the numbers that can be written as the sum of fifth powers of their digits is: {}",
        sum
    );
}

fn find_sum_of_fifth_power_digit_numbers() -> u32 {
    let mut sum = 0;

    for i in 10..=354_294 {
        if is_sum_of_fifth_power_of_digits(i) {
            sum += i;
        }
    }

    sum
}

fn is_sum_of_fifth_power_of_digits(number: u32) -> bool {
    let digits = number.to_string();
    let sum_of_fifth_power_digits: u32 = digits.chars().map(|c| c.to_digit(10).unwrap().pow(5)).sum();

    sum_of_fifth_power_digits == number
}
