use std::io;
use std::time::Instant;

mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;
mod problem_6;
mod problem_8;
mod problem_9;
mod problem_7;
mod problem_10;
mod problem_11;
mod problem_12;
mod problem_13;
mod problem_14;
mod problem_15;
mod problem_16;
mod problem_17;
mod problem_18;


fn main() {
    println!("Project Euler");
    let mut problem_number = String::new();
    println!("Enter a problem number:");

    io::stdin().read_line(&mut problem_number).expect("Failed to read line");

    let start_time = Instant::now();

    match problem_number.trim() {
        "1" => problem_1::solve(),
        "2" => problem_2::solve(),
        "3" => problem_3::solve(),
        "4" => problem_4::solve(),
        "5" => problem_5::solve(),
        "6" => problem_6::solve(),
        "7" => problem_7::solve(),
        "8" => problem_8::solve(),
        "9" => problem_9::solve(),
        "10" => problem_10::solve(),
        "11" => problem_11::solve(),
        "12" => problem_12::solve(),
        "13" => problem_13::solve(),
        "14" => problem_14::solve(),
        "15" => problem_15::solve(),
        "16" => problem_16::solve(),
        "17" => problem_17::solve(),
        "18" => problem_18::solve(),
        _ => println!("Invalid problem number"),
    }

    let elapsed = start_time.elapsed(); // Calculate the elapsed time

    println!("Done! It took {:?} to solve the problem.", elapsed);
}

