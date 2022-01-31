use regex::Regex;
use std::io;

fn read_date() -> String {
    let mut date = String::new();

    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");

    date = date.trim().to_string();

    date
}

fn main() {
    println!("Please input your date of birth in format YYYY-MM-DD to calculate age.");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    let date = loop {
        let date = read_date();
        if date.len() == 0 {
            println!("Please, your date of birth is needed to calculate age.");
            continue;
        }
        if re.is_match(&date) {
            break date;
        }
        println!("Enter your date of birth like 2014-01-01.");
    };

    println!("Your date of birth: '{}'", date);
}
