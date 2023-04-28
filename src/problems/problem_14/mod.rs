pub fn solve() {
    let limit = 1_000_000;
    let mut max_length = 0;
    let mut max_starting_number = 0;

    for starting_number in 1..limit {
        let length = collatz_sequence_length(starting_number);
        if length > max_length {
            max_length = length;
            max_starting_number = starting_number;
        }
    }

    println!("The starting number with the longest Collatz sequence is: {}", max_starting_number);
}

fn collatz_sequence_length(starting_number: u64) -> u64 {
    let mut length = 1;
    let mut n = starting_number;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }

    length
}
