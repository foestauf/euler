mod sub;

// Function to find the 10001st prime number
pub fn solve() {
    let mut primes = vec![2];
    let mut i = 3;
    while primes.len() < 10001 {
        if sub::is_prime(i) {
            primes.push(i);
        }
        i += 2;
    }
    println!("The 10001st prime number is {}", primes[10000]);
}