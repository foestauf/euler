pub fn solve() {
    let limit = 28_123;
    let abundant_numbers = find_abundant_numbers(limit);
    let non_abundant_sum_numbers: Vec<u32> = find_non_abundant_sum_numbers(limit, &abundant_numbers);

    let total: u32 = non_abundant_sum_numbers.iter().sum();
    println!("The sum of all positive integers that cannot be written as the sum of two abundant numbers is: {}", total);
}

fn find_abundant_numbers(limit: u32) -> Vec<u32> {
    (1..=limit).filter(|&n| sum_of_divisors(n) > n).collect()
}

fn sum_of_divisors(n: u32) -> u32 {
    (1..=(n / 2)).filter(|&i| n % i == 0).sum()
}

fn find_non_abundant_sum_numbers(limit: u32, abundant_numbers: &[u32]) -> Vec<u32> {
    let mut can_be_written_as_abundant_sum = vec![false; (limit + 1) as usize];

    for &a in abundant_numbers {
        for &b in abundant_numbers {
            let sum = a + b;
            if sum <= limit {
                can_be_written_as_abundant_sum[sum as usize] = true;
            } else {
                break;
            }
        }
    }

    (1..=limit)
        .filter(|&n| !can_be_written_as_abundant_sum[n as usize])
        .collect()
}
