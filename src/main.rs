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
    
    println!("How many Events do you want to input?");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Couldn't read from stdin");
    let input:i32 = line.trim().parse().expect("Input was not an integer.");

    for i in 0..input{

        let discription = helpers::input_string_with_message("Please input the name of the event");
        println!("Event {} is named: {}", i+1, discription);
        let day = helpers::input_int_with_message("Please input the day of the event");
        println!("Day of event {} is: {}", i+1, day);
        let month = helpers::input_int_with_message("Please input the month of the event");
        println!("Month of event {} is: {}", i+1, month);
        let year = helpers::input_int_with_message("Please input the year of the event");
        println!("Year of event {} is: {}", i+1, year);

    }


}
