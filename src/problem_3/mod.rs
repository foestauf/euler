pub fn solve() {
    let mut n: i64 = 600851475143;
    let original_n = n; // Store the original value of n
    let mut largest_prime_factor = 0;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            // Check if i is prime
            let mut is_prime = true;
            let mut j = 2;
            while j * j <= i {
                if i % j == 0 {
                    is_prime = false;
                    break;
                }
                j += 1;
            }

            // Update largest_prime_factor if i is prime
            if is_prime {
                largest_prime_factor = i;
            }

            // Update n
            let mut temp_n = n / i;
            while temp_n % i == 0 {
                temp_n /= i;
            }
            n = temp_n;
        }
        i += 1;
    }

    if n > largest_prime_factor {
        largest_prime_factor = n;
    }

    println!("Largest prime factor of {} is {}", original_n, largest_prime_factor);
}
