mod is_palindrome;

pub fn solve() {
    println!("Problem 4");
    let mut largest_palindrome = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_palindrome::is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }
    println!("Largest palindrome: {}", largest_palindrome);
}

