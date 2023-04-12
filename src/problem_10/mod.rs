pub fn solve() {
    let limit = 2000000;
    let sum_of_primes = primes_below(limit);
    println!("The sum of all primes below {} is {}", limit, sum_of_primes);
}

fn primes_below(limit: usize) -> usize {
    let mut primes = vec![true; limit];
    primes[0] = false;
    primes[1] = false;
    for i in 2..limit {
        if primes[i] {
            let mut j = i * i;
            while j < limit {
                primes[j] = false;
                j += i;
            }
        }
    }
    let mut sum = 0;
    for i in 2..limit {
        if primes[i] {
            sum += i;
        }
    }
    sum
}