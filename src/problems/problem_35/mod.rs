use primal::Sieve;

fn rotations(n: usize) -> Vec<usize> {
    let s = n.to_string();
    (0..s.len()).map(|i| (&s[i..], &s[..i])).map(|(a, b)| format!("{}{}", a, b).parse().unwrap()).collect()
}

pub fn solve(){
    let sieve = Sieve::new(1_000_000);
    let circular_primes = (2..1_000_000).filter(|&n| rotations(n).iter().all(|&m| sieve.is_prime(m))).count();
    println!("{}", circular_primes);
}
