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
    println!("Please input your date of birth to calculate age.");

    let date = loop {
        let date = read_date();
        if date.len() > 0 {
            break date;
        }

        println!("Please, your date of birth is needed to calculate age.");
    };

    println!("input '{}'", date);

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
