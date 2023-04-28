pub fn solve() {
    let mut count = 0;
    let mut day_of_week = 2; // January 1, 1901 was a Tuesday

    for year in 1901..=2000 {
        for month in 1..=12 {
            if day_of_week == 0 { // 0 represents Sunday
                count += 1;
            }
            day_of_week = (day_of_week + days_in_month(month, year)) % 7;
        }
    }

    println!("Number of Sundays that fell on the first of the month during the twentieth century: {}", count);
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(month: u32, year: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    }
}
