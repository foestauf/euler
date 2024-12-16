use std::fs;
use std::collections::HashSet;

pub fn solve() {
    // Read the entire file, which contains comma-separated words in quotes
    let content = fs::read_to_string("words.txt")
        .expect("Failed to read file");

    // Split on commas, trim surrounding quotes
    let words: Vec<String> = content
        .split(',')
        .map(|w| w.trim_matches('"'))
        .map(String::from)
        .collect();

    // Precompute a set of triangle numbers up to a reasonable max
    // (For ~20 chars max word length, 20*26 = 520)
    let max_sum = 520;
    let mut triangle_numbers = HashSet::new();
    let mut n = 1;
    loop {
        let t = n * (n + 1) / 2;
        if t > max_sum {
            break;
        }
        triangle_numbers.insert(t);
        n += 1;
    }

    let mut count = 0;
    for word in words {
        // Sum of alphabetical positions (A=1, B=2, etc.)
        let sum: u32 = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_uppercase() as u32 - 'A' as u32 + 1)
            .sum();

        // Check if sum is a triangle number
        if triangle_numbers.contains(&sum) {
            count += 1;
        }
    }

    println!("Number of triangle words: {}", count);
}
