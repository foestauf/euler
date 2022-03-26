use std::io;
mod problem_1;

fn main() {
    println!("Project Euler");
    let mut problem_number = String::new();
    println!("Enter a problem number:");

    io::stdin().read_line(&mut problem_number).expect("Failed to read line");

    match problem_number.trim() {
        "1" => problem_1::solve(),
        _ => println!("Invalid problem number"),
    }

    println!("Done!");
}

