mod sub;

// Function to find the difference between the sum of the squares of the first n natural numbers and the square of the sum of the first n natural numbers
pub fn solve() {
    let n = 100;
    let sum_of_squares = sub::sum_of_squares(n);
    let square_of_sum = sub::square_of_sum(n);
    let difference = square_of_sum - sum_of_squares;
    println!("The difference between the sum of the squares of the first {} natural numbers and the square of the sum is {}", n, difference);
}