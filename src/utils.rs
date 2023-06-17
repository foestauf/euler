pub fn is_pandigital(number: &str) -> bool {
    if number.len() != 9 {
        return false;
    }
    let mut digits = [false; 9];
    for ch in number.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        if digit == 0 || digits[digit - 1] {
            return false;
        }
        digits[digit - 1] = true;
    }
    true
}