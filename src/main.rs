use std::io;
mod problem_1;
mod problem_2;
mod problem_3;

fn main() {
    println!("Project Euler");
    let mut problem_number = String::new();
    println!("Enter a problem number:");

    io::stdin().read_line(&mut problem_number).expect("Failed to read line");

    match problem_number.trim() {
        "1" => problem_1::solve(),
        "2" => problem_2::solve(),
        "3" => problem_3::solve(),
        _ => println!("Invalid problem number"),
    }

    println!("Done!");
}

