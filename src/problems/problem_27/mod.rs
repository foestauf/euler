pub fn solve() {
    let (a, b) = find_coefficients_with_max_primes(1000, 1000);
    println!("The product of coefficients a and b is: {}", a * b);
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_coefficients_with_max_primes(a_limit: i32, b_limit: i32) -> (i32, i32) {
    let mut max_primes = 0;
    let mut max_a = 0;
    let mut max_b = 0;

    for a in -a_limit + 1..a_limit {
        for b in -b_limit + 1..b_limit {
            let mut n = 0;
            while is_prime(n * n + a * n + b) {
                n += 1;
            }

            if n > max_primes {
                max_primes = n;
                max_a = a;
                max_b = b;
            }
        }
    }

    (max_a, max_b)
}
