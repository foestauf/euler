// Function to find the sum of the squares of the first n natural numbers
pub(crate) fn sum_of_squares(n: u64) -> u64 {
    (1..n + 1).map(|x| x * x).sum()
}

// Function to find the square of the sum of the first n natural numbers
pub(crate) fn square_of_sum(n: u64) -> u64 {
    let sum: u64 = (1..n + 1).sum();
    sum * sum
}