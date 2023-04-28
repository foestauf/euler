pub fn solve() {
    let sum_of_amicable_numbers = find_sum_of_amicable_numbers(10000);
    println!("The sum of all amicable numbers under 10,000 is: {}", sum_of_amicable_numbers);
}

fn find_sum_of_amicable_numbers(limit: u32) -> u32 {
    let mut sum = 0;

    for a in 1..limit {
        let b = sum_of_divisors(a);
        if a != b && sum_of_divisors(b) == a {
            sum += a;
        }
    }

    sum
}

fn sum_of_divisors(n: u32) -> u32 {
    (1..=(n / 2)).filter(|&i| n % i == 0).sum()
}
