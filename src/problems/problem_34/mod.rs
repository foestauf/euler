pub fn solve() {
    let factorials: Vec<u32> = (0..=9).map(factorial).collect();
    let upper_bound = factorials[9] * 7; // 7 * 9! is the largest possible number to consider

    let result: u32 = (10..upper_bound)
        .filter(|&n| n == sum_of_digit_factorials(n))
        .sum();

    println!("The sum of all numbers equal to the sum of the factorial of their digits is: {}", result);
}

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

fn sum_of_digit_factorials(number: u32) -> u32 {
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(factorial)
        .sum()
}


