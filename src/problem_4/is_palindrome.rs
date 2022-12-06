pub(crate) fn is_palindrome(n: u32) -> bool {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let mut i = 0;
    let mut j = digits.len() - 1;
    while i < j {
        if digits[i] != digits[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}