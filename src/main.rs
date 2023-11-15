use chrono::*;
//use chrono_tz::Europe::Berlin;
use icalendar::*;

mod helpers;

fn main() {

    println!("Hello do you want to create a calendar?[y/n]");

    
    loop{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Couldn't read from stdin");
        let input = helpers::match_input(&line);
        match input {
            Ok(v) => {
                if v == helpers::Return::Yes { 
                    break
                }
                if v == helpers::Return::No { 
                    println!("Okay bye");
                    return
                }
            },
            Err(e) => println!("{e}"),
        }
    }  
    let calendar_name = helpers::input_string_with_message("Whats the name of the calendar?");

    let mut my_calendar = Calendar::new()
        .name(&calendar_name).done();

    let number_event = helpers::input_int_with_message("How many Events do you want to input?");

    for _ in 0..number_event{

        let discription = helpers::input_string_with_message("Please input the name of the event"); 
        let day = helpers::input_int_with_message("Please input the day of the event") as u32;
        let month = helpers::input_int_with_message("Please input the month of the event") as u32;
        let year = helpers::input_int_with_message("Please input the year of the event");

        my_calendar.push(
            Event::new()
                .starts(Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap())
                .description(&discription)
                .done(),
        );

    }

    println!("{}", my_calendar);

}
