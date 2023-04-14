pub fn solve() {
    let limit = 1000;
    let mut letter_count = 0;

    for i in 1..=limit {
        letter_count += get_word_length(i);
    }

    println!("The total number of letters used from 1 to {} is: {}", limit, letter_count);
}

fn get_word_length(n: u32) -> u32 {
    let mut word_length = 0;

    let ones = n % 10;
    let tens = (n / 10) % 10;
    let hundreds = (n / 100) % 10;
    let thousands = n / 1000;

    // Calculate the length of ones
    if tens != 1 {
        match ones {
            1 | 2 | 6 => word_length += 3,
            4 | 5 | 9 => word_length += 4,
            3 | 7 | 8 => word_length += 5,
            _ => {}
        }
    }

    // Calculate the length of tens
    if tens == 1 {
        word_length += match ones {
            0 => 3, // ten
            1 | 2 => 6, // eleven, twelve
            5 | 6 => 7, // fifteen, sixteen
            3 | 4 | 8 | 9 => 8, // thirteen, fourteen, eighteen, nineteen
            7 => 9, // seventeen
            _ => unreachable!(),
        };
    } else {
        match tens {
            2 | 3 | 8 | 9 => word_length += 6,
            4 | 5 | 6 => word_length += 5,
            7 => word_length += 7,
            _ => {}
        }
    }

    // Calculate the length of hundreds
    if hundreds > 0 {
        match hundreds {
            1 | 2 | 6 => word_length += 3,
            4 | 5 | 9 => word_length += 4,
            3 | 7 | 8 => word_length += 5,
            _ => {}
        }
        word_length += 7; // "hundred"
        if n % 100 != 0 {
            word_length += 3; // "and"
        }
    }

    // Calculate the length of thousands
    if thousands > 0 {
        word_length += 11; // "one thousand"
    }

    word_length
}


