#[derive(PartialEq)]
#[derive(Debug)]
pub enum Return {Yes,No}

pub fn match_input(input: &str) -> Result<Return,&'static str>{
    match input.to_lowercase().trim(){
        "y" => return Ok(Return::Yes),
        "n" => return Ok(Return::No),
        _ => return Err("Wrong Input, try again.\nPossible answers are [y/n]!"),
    }
}

#[allow(dead_code)]
pub fn input_string() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Couldn't read from stdin");
    line
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

pub fn parse_string_date_to_int(date: String) -> (i32,i32,i32) {
    

    (1,1,1)
}
 