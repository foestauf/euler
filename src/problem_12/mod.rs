pub fn solve() {
    let divisors_threshold = 500;
    let triangle_number = find_triangle_number(divisors_threshold);
    println!(
        "The first triangle number with over {} divisors is: {}",
        divisors_threshold, triangle_number
    );
}

fn find_triangle_number(divisors_threshold: u32) -> u64 {
    let mut triangle_number = 0;
    let mut index: u64 = 1;

    while count_divisors(triangle_number) <= divisors_threshold {
        triangle_number += index;
        index += 1;
    }

    triangle_number
}

fn count_divisors(number: u64) -> u32 {
    if number == 0 {
        return 0;
    }

    let mut count = 0;
    let mut i = 1;

    while i * i < number {
        if number % i == 0 {
            count += 2; // both i and number / i are divisors
        }
        i += 1;
    }

    if i * i == number {
        count += 1; // the square root is also a divisor
    }

    count
}
