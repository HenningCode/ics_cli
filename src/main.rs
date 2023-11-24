use chrono_tz::Europe::Berlin;
use icalendar::*;
use std::{fs::File, io::Write};

mod helpers;

fn main() {
    println!("Hello do you want to create a calendar?[y/n]");

    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Couldn't read from stdin");
        let input = helpers::confirm_input(&line);
        match input {
            Some(v) => {
                if !v {
                    println!("Okay bye");
                    return;
                }
                break;
            }
            None => println!("Wrong Input, try again.\nPossible answers are [y/n]!"),
        }
    }

    let calendar_name = helpers::input_string_with_message("Whats the name of the calendar?");
    let number_event = helpers::input_int_with_message("How many Events do you want to input?");

    let mut my_calendar = Calendar::new().name(&calendar_name).done();
    let file_name = format!("{calendar_name}.ical");

    for _ in 0..number_event {
        let discription = helpers::input_string_with_message("Please input the name of the event");
        let (day, month, year) = helpers::get_date();
        let (hours, minutes) = helpers::get_event_time();
        my_calendar.push(
            Event::new()
                .starts(
                    CalendarDateTime::from_ymd_hm_tzid(year, month, day, hours, minutes, Berlin)
                        .unwrap(),
                )
                .summary(&discription)
                .done(),
        );
    }

    let file_result = File::create(file_name.clone());
    let mut file = match file_result {
        Ok(f) => f,
        Err(_) => File::open(file_name).unwrap(),
    };

    file.write_all(my_calendar.to_string().as_bytes())
        .expect("Writing did not work!");
    println!("{my_calendar}");
}
