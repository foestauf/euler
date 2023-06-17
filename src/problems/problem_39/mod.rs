pub fn solve() {
    let mut max_count = 0;
    let mut max_p = 0;

    for p in 1..=1000 {
        let mut count = 0;
        for a in 1..p {
            for b in a..(p - a) {
                let c = p - a - b;
                if a * a + b * b == c * c {
                    count += 1;
                }
            }
        }
        if count > max_count {
            max_count = count;
            max_p = p;
        }
    }

    println!("The value of p ≤ 1000 for which the number of solutions is maximised is: {}", max_p);
}
