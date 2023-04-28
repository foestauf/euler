pub fn solve() {
    let sum = sum_diagonal_numbers_of_spiral(1001);
    println!("The sum of the diagonal numbers in a 1001x1001 spiral is: {}", sum);
}

fn sum_diagonal_numbers_of_spiral(size: usize) -> usize {
    if size % 2 == 0 {
        panic!("Size should be odd");
    }

    let mut sum = 1;
    let mut current_number = 1;

    for step in (3..=size).step_by(2) {
        for _ in 0..4 {
            current_number += step - 1;
            sum += current_number;
        }
    }

    sum
}
