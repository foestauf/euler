pub fn solve() {
    let digits = "0123456789";
    let target_permutation = 1_000_000;

    let result = find_lexicographic_permutation(digits, target_permutation);
    println!("The millionth lexicographic permutation of the digits 0 through 9 is: {}", result);
}

fn find_lexicographic_permutation(digits: &str, target_permutation: usize) -> String {
    let mut digits = digits.chars().collect::<Vec<char>>();
    let mut result = String::new();
    let mut remaining_permutations = target_permutation - 1;
    let factorial = |n: u32| (1..=n).map(|x| x as usize).product::<usize>(); // Cast each element to usize before multiplying

    for i in (0..digits.len()).rev() {
        let f = factorial(i as u32);
        let index = remaining_permutations / f;
        remaining_permutations %= f;

        result.push(digits[index]);
        digits.remove(index);

        if remaining_permutations == 0 {
            result.extend(digits.into_iter());
            break;
        }
    }

    result
}

