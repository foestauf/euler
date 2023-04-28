// Solve for the smallest multiple that can be divided by all numbers from 1 to 20

mod sub;

pub(crate) fn solve() {
    let mut number = 20;
    let mut found = false;
    while !found {
        if sub::is_divisible_by_all(number) {
            found = true;
        } else {
            number += 20;
        }
    }
    println!("The smallest number divisible by all numbers from 1 to 20 is {}", number);
}