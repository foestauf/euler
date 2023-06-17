pub fn solve() {
    let mut s = String::new();
    let mut i = 1;

    while s.len() <= 1_000_000 {
        s.push_str(&i.to_string());
        i += 1;
    }

    let d = |i: usize| s.chars().nth(i - 1).unwrap().to_digit(10).unwrap() as usize;

    let product = d(1) * d(10) * d(100) * d(1_000) * d(10_000) * d(100_000) * d(1_000_000);

    println!("The product is: {}", product);
}
