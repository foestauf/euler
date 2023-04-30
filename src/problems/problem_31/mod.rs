pub fn solve() {
    let target = 200;
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];

    let combinations = count_combinations(target, &coins);
    println!("Number of different ways £2 can be made using any number of coins: {}", combinations);
}

fn count_combinations(target: usize, coins: &[usize]) -> usize {
    let mut ways = vec![0; target + 1];
    ways[0] = 1;

    for &coin in coins {
        for i in coin..=target {
            ways[i] += ways[i - coin];
        }
    }

    ways[target]
}
