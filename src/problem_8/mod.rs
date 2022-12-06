// Function to find the pythagorean triplet for which a + b +c = 1000
pub fn solve() {
    let mut result = 0;
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if a + b + c == 1000 && (a as i32).pow(2) + (b as i32).pow(2) == (c as i32).pow(2) {
                    result = a * b * c;
                }
            }
        }
    }
    // print the result
    println!("The result is {}", result);
}