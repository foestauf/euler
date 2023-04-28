pub fn solve() {
    let limit = 1000;
    let d = find_longest_recurring_cycle(limit);

    println!("The value of d < {} with the longest recurring cycle is: {}", limit, d);
}

fn find_longest_recurring_cycle(limit: usize) -> usize {
    let mut longest_cycle = 0;
    let mut d_with_longest_cycle = 0;

    for d in 1..limit {
        let cycle_length = calculate_cycle_length(d);
        if cycle_length > longest_cycle {
            longest_cycle = cycle_length;
            d_with_longest_cycle = d;
        }
    }

    d_with_longest_cycle
}

fn calculate_cycle_length(d: usize) -> usize {
    let mut remainders = vec![0; d + 1];
    let mut value = 1;
    let mut position = 0;

    while remainders[value] == 0 && value != 0 {
        remainders[value] = position;
        value *= 10;
        value %= d;
        position += 1;
    }

    if value == 0 {
        0
    } else {
        position - remainders[value]
    }
}