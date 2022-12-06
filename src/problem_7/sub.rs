// Function to determine if number is prime
pub(crate) fn is_prime(n: i32) -> bool {
    let mut i = 3;
    while i <= (n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}