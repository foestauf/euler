use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve() {
    let names = read_names_from_file("names.txt").unwrap();
    let total_score = calculate_total_score(names);
    println!("The total score of all names in the file is: {}", total_score);
}

fn read_names_from_file(file_name: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(file_name);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let names = reader
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|line| line.split(',').map(str::to_owned).collect::<Vec<String>>())
        .collect::<Vec<String>>();

    Ok(names)
}

fn calculate_total_score(mut names: Vec<String>) -> u32 {
    names.sort();

    names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let alphabetical_value = name
                .chars()
                .filter(|&c| c != '"')
                .map(|c| c as u32 - 'A' as u32 + 1)
                .sum::<u32>();
            (i as u32 + 1) * alphabetical_value
        })
        .sum()
}
