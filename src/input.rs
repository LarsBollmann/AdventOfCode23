use dotenv::dotenv;
use reqwest::blocking::Client;
use core::panic;
use std::{env, fs};

pub fn get_input(day: u8) -> String {
    let input_file = format!("data/inputs/{}.txt", day);
    if let Ok(input) = fs::read_to_string(input_file) {
        return input;
    }

    dotenv().ok();

    let client = Client::new();

    let input = client
        .get(format!("https://adventofcode.com/2023/day/{}/input", day))
        .header(
            "Cookie",
            format!(
                "session={}",
                env::var("AOC_SESSION").expect("Could not find session environment variable")
            ),
        )
        .send()
        .expect("Error getting input")
        .text()
        .expect("Error parsing input");

    fs::create_dir_all("data/inputs").expect("Error creating inputs directory");
    fs::write(format!("data/inputs/{}.txt", day), &input).expect("Error writing input");

    input
}

pub fn get_example(day: u8) -> String {
    let input_file = format!("data/examples/{}.txt", day);
    let input = {
        if let Ok(input) = fs::read_to_string(input_file) {
            input
        } else {
            fs::create_dir_all("data/examples").expect("Error creating examples directory");
            fs::write(format!("data/examples/{}.txt", day), "").expect("Error writing input");
            String::from("")
        }
    };
    match input.is_empty() {
        true => {
            panic!("\n\nERROR: No example input found for day {}. Paste the example input into data/examples/{}.txt", day, day);
        }
        false => input,
    }
}
