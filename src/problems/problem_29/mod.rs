use num_bigint::BigUint;
use num_traits::Pow;
use std::collections::HashSet;

pub fn solve() {
    let distinct_terms_count = count_distinct_terms(100);
    println!(
        "The number of distinct terms in the sequence a^b for 2 <= a <= 100 and 2 <= b <= 100 is: {}",
        distinct_terms_count
    );
}

fn count_distinct_terms(limit: u64) -> usize {
    let mut distinct_terms = HashSet::new();

    for a in 2..=limit {
        for b in 2..=limit {
            let big_a = BigUint::from(a);
            let term = big_a.pow(b as u32);
            distinct_terms.insert(term);
        }
    }

    distinct_terms.len()
}
