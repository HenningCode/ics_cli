use chrono_tz::Europe::Berlin;
use icalendar::*;
use std::{fs::File, io::Write};
use regex::Regex;

mod helpers;

fn main() {
    
    let re = Regex::new(r"\d{2}\.\d{2}\.\d{4}").unwrap();
    println!("Hello do you want to create a calendar?[y/n]");

    
    loop{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Couldn't read from stdin");
        let input = helpers::match_input(&line);
        match input {
            Ok(v) => {
                if v == helpers::Return::Yes { 
                    break;
                }
                if v == helpers::Return::No { 
                    println!("Okay bye");
                    return;
                }
            },
            Err(e) => println!("{e}"),
        }
    }  

    let calendar_name = helpers::input_string_with_message("Whats the name of the calendar?");
    let number_event = helpers::input_int_with_message("How many Events do you want to input?");

    let mut my_calendar = Calendar::new()
        .name(&calendar_name).done();
    let file_name = format!("{calendar_name}.ical");

    for _ in 0..number_event{

        let discription = helpers::input_string_with_message("Please input the name of the event");
        let mut date = helpers::input_string_with_message("Whats the date of the event? (dd.mm.yyyy)");
        loop {
            let valid = re.is_match(&date);
            if valid {
                break;
            }
            date = helpers::input_string_with_message("Wrong input format. Try again! (dd.mm.yyyy)");
        } 

        let (day, month, year) = helpers::parse_string_date_to_int(date);

        my_calendar.push(
            Event::new()
                .starts(CalendarDateTime::from_ymd_hm_tzid(year, month, day, 20, 15, Berlin).unwrap())
                .summary(&discription)
                .done(),
        );

    }

    let file_result = File::create(file_name.clone());
    let mut file = match file_result {
        Ok(f) => f,
        Err(_) => File::open(file_name).unwrap(),
    };

    file.write_all(my_calendar.to_string().as_bytes()).expect("Writing did not work!");
    println!("{}", my_calendar);
    
}

