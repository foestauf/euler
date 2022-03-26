pub fn solve() {
    let mut sum: u32 = 0;
    for i in 0..1000 {
        let three_modulus: u32 = i % 3;
        let five_modulus: u32 = i % 5;
        if three_modulus == 0 {
            sum = sum + i;
            continue;
        }
        if five_modulus == 0 {
            sum = sum + i;
            continue;
        }
    }
    println!("sum is {}", sum);
}
