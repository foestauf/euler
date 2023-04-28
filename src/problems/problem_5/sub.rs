// Function to check if number is divisible by all numbers from 1 to 20
pub fn is_divisible_by_all(number: u64) -> bool {
    let mut is_divisible = true;
    for i in 1..21 {
        if number % i != 0 {
            is_divisible = false;
            break;
        }
    }
    is_divisible
}