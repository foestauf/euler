pub fn solve() {
    let result: u32 = (1..1_000_000)
        .filter(|&n| is_palindrome(n.to_string()) && is_palindrome(format!("{:b}", n)))
        .sum();

    println!("The sum of all numbers, less than one million, which are palindromic in base 10 and base 2 is {}", result);
}

fn is_palindrome(s: String) -> bool {
    s == s.chars().rev().collect::<String>()
}
