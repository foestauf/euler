use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn solve() {
    let numbers = read_large_numbers_from_file("numbers.txt").unwrap();
    let sum = find_sum_of_large_numbers(&numbers);
    let first_ten_digits = &sum[0..10];
    println!("The first ten digits of the sum are: {}", first_ten_digits);
}

fn find_sum_of_large_numbers(numbers: &[String]) -> String {
    let mut sum = num_bigint::BigUint::from(0u32);

    for number in numbers {
        let n = num_bigint::BigUint::parse_bytes(number.as_bytes(), 10).unwrap();
        sum += n;
    }

    let sum_string = sum.to_string();
    sum_string
}

fn read_large_numbers_from_file(file_name: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(file_name);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let numbers: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string()) // Trim whitespace characters from each line
        .collect();

    Ok(numbers)
}

