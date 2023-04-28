use std::io;
use std::time::Instant;

mod problems;



fn main() {
    println!("Project Euler");
    let mut problem_number = String::new();
    println!("Enter a problem number:");

    io::stdin().read_line(&mut problem_number).expect("Failed to read line");

    let start_time = Instant::now();

    problems::run_problem(problem_number.trim());

    let elapsed = start_time.elapsed(); // Calculate the elapsed time

    println!("Done! It took {:?} to solve the problem.", elapsed);
}

