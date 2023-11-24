use regex::Regex;

pub fn confirm_input(input: &str) -> Option<bool> {
    match input.to_lowercase().trim() {
        "y" | "Y" | "j" | "J" => return Some(true),
        "n" | "N" => return Some(false),
        _ => return None,
    }
}

#[allow(dead_code)]
pub fn input_string() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Couldn't read from stdin");
    line.trim().to_string()
}

pub fn input_string_with_message(msg: &str) -> String {
    println!("{msg}");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Couldn't read from stdin");
    line.trim().to_string()
}

pub fn input_int_with_message(msg: &str) -> i32 {
    println!("{msg}");
    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Couldn't read from stdin");
        let parsed_int_result = line.trim().parse(); // Getting the error here
        match parsed_int_result {
            Ok(integer) => {
                return integer;
            }
            Err(_) => println!("Input was not an integer. Try again:"),
        };
    }
}

pub fn input_with_regex_validation(msg: &str, msg_err: &str, re: &Regex) -> String {
    let mut date = input_string_with_message(msg);
    loop {
        let valid = re.is_match(&date);
        if valid {
            return date;
        }
        date = input_string_with_message(msg_err);
    }
}

pub fn parse_string_date_to_int(date: &String) -> (u32, u32, i32) {
    let d = date.as_str();
    let day: u32 = d[..2]
        .to_string()
        .parse()
        .expect("this should already be verified ");
    let month: u32 = d[3..5]
        .to_string()
        .parse()
        .expect("this should already be verified ");
    let year: i32 = d[6..10]
        .to_string()
        .parse()
        .expect("this should already be verified ");
    (day, month, year)
}

pub fn parse_string_time_to_int(date: &String) -> (u32, u32) {
    let d = date.as_str();
    let hours: u32 = d[..2]
        .to_string()
        .parse()
        .expect("this should already be verified ");
    let minutes: u32 = d[3..5]
        .to_string()
        .parse()
        .expect("this should already be verified ");
    (hours, minutes)
}

pub fn validate_in_range_u32(value: u32, min: u32, max: u32) -> bool {
    if value <= min || value >= max {
        return false;
    }
    true
}

pub fn validate_in_range_i32(value: i32, min: i32, max: i32) -> bool {
    if value <= min || value >= max {
        return false;
    }
    true
}

pub fn calc_feb_days(year: i32) -> u32 {
    if year % 400 == 0 {
        return 29;
    }
    if year % 4 == 0 && year % 100 !=0 {
        return 28;
    }
    28
}

pub fn get_date() -> (u32, u32, i32) {
    let re_date: Regex = Regex::new(r"^\d{2}\.\d{2}\.\d{4}$").unwrap();

    let (mut day, mut month, mut year);
    loop {
        let date = input_with_regex_validation(
            "Whats the date of the event? (dd.mm.yyyy)",
            "Wrong input format. Try again! (dd.mm.yyyy)",
            &re_date,
        );
        (day, month, year) = parse_string_date_to_int(&date);

        if !validate_in_range_i32(year, 0, 3000) {
            println!("Year is out of range (0-3000");
            continue;
        }

        if !validate_in_range_u32(month, 1, 12) {
            println!("Month is out of range (1-12)");
            continue;
        }
        let max = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => calc_feb_days(year),
            4 | 6 | 9 | 11 => 30,
            _ => panic!("Should not happen"),
        };

        if !validate_in_range_u32(day, 1, max) {
            println!("Day is out of range (1-{max})");
            continue;
        }
        break;
    }
    (day, month, year)
}

pub fn get_event_time() -> (u32, u32) {
    let re: Regex = Regex::new(r"^\d{2}:\d{2}").unwrap();

    let (mut hours, mut minutes);
    loop {
        let time = input_with_regex_validation(
            "Whats the time of the event? (hh:mm)",
            "Wrong input format. Try again! (hh:mm)",
            &re,
        );
        (hours, minutes) = parse_string_time_to_int(&time);

        if !validate_in_range_u32(hours, 0, 23) {
            println!("Month is out of range (1-12)");
            continue;
        }

        if !validate_in_range_u32(minutes, 0, 59) {
            println!("Day is out of range (0-59)");
            continue;
        }
        break;
    }
    (hours, minutes)
}


