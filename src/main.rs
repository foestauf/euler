use std::io;
mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;
mod problem_6;
mod problem_7;
mod problem_8;

fn main() {
    println!("Project Euler");
    let mut problem_number = String::new();
    println!("Enter a problem number:");

    io::stdin().read_line(&mut problem_number).expect("Failed to read line");

    match problem_number.trim() {
        "1" => problem_1::solve(),
        "2" => problem_2::solve(),
        "3" => problem_3::solve(),
        "4" => problem_4::solve(),
        "5" => problem_5::solve(),
        "6" => problem_6::solve(),
        "7" => problem_7::solve(),
        "8" => problem_8::solve(),
        _ => println!("Invalid problem number"),
    }

    println!("Done!");
}

