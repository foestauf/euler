pub fn solve() {
    let mut sum: u32 = 0;
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    // sum a and b
    let mut c: u32 = a + b;
    while c < 4000000 {
        if c % 2 == 0 {
            sum += c;
        }
        a = b;
        b = c;
        c = a + b;
    }
    println!("sum is {}", sum);
}