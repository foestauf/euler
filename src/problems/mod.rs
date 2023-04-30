pub mod problem_1;
pub mod problem_2;
pub mod problem_3;
pub mod problem_4;
pub mod problem_5;
pub mod problem_6;
pub mod problem_8;
pub mod problem_9;
pub mod problem_7;
pub mod problem_10;
pub mod problem_11;
pub mod problem_12;
pub mod problem_13;
pub mod problem_14;
pub mod problem_15;
pub mod problem_16;
pub mod problem_17;
pub mod problem_18;
pub mod problem_19;
pub mod problem_20;
pub mod problem_21;
pub mod problem_22;
pub mod problem_23;
pub mod problem_24;
pub mod problem_25;
pub mod problem_26;
pub mod problem_27;
pub mod problem_28;
pub mod problem_29;
pub mod problem_30;
pub mod problem_31;
pub mod problem_32;
pub mod problem_33;
pub mod problem_34;



pub fn run_problem(number: &str) {
    match number {
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
        "19" => problem_19::solve(),
        "20" => problem_20::solve(),
        "21" => problem_21::solve(),
        "22" => problem_22::solve(),
        "23" => problem_23::solve(),
        "24" => problem_24::solve(),
        "25" => problem_25::solve(),
        "26" => problem_26::solve(),
        "27" => problem_27::solve(),
        "28" => problem_28::solve(),
        "29" => problem_29::solve(),
        "30" => problem_30::solve(),
        "31" => problem_31::solve(),
        "32" => problem_32::solve(),
        "33" => problem_33::solve(),
        "34" => problem_34::solve(),
        _ => println!("Invalid problem number"),
    }
}
