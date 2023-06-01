use std::collections::HashSet;

pub fn solve() {
    let limit = 1_000_000;
    let primes = generate_primes(limit);
    let truncatable_primes: Vec<u32> = primes.iter().cloned().filter(|&p| is_truncatable(p, &primes)).collect();

    let result: u32 = truncatable_primes.iter().sum();

    println!("The sum of all truncatable primes is: {}", result);
}

fn generate_primes(limit: u32) -> HashSet<u32> {
    let mut primes = HashSet::new();
    let mut sieve = vec![true; limit as usize];
    sieve[0] = false;
    sieve[1] = false;

    for number in 2..limit {
        if sieve[number as usize] {
            primes.insert(number);
            let mut multiple = number * 2;
            while multiple < limit {
                sieve[multiple as usize] = false;
                multiple += number;
            }
        }
    }

    primes
}

fn is_truncatable(n: u32, primes: &HashSet<u32>) -> bool {
    let digits: Vec<_> = n.to_string().chars().collect();
    let len = digits.len();

    // We exclude primes with only one digit
    if len == 1 {
        return false;
    }

    // Check truncations from the left
    for i in 1..len {
        let trunc = digits[i..].iter().collect::<String>().parse::<u32>().unwrap();
        if !primes.contains(&trunc) {
            return false;
        }
    }

    // Check truncations from the right
    for i in 1..len {
        let trunc = digits[..len - i].iter().collect::<String>().parse::<u32>().unwrap();
        if !primes.contains(&trunc) {
            return false;
        }
    }

    true
}
