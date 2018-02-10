use std::io;
use std::process;

use std::error::Error;

mod check;

fn read_line() -> Result<String, String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(error) => Err(format!("{}", error.description())),
        Ok(_) => Ok(input),
    }
}

fn parse_number(input: String) -> Result<u32, String> {
    match input.parse::<u32>() {
        Err(error) => Err(format!("{}", error.description())),
        Ok(int) => Ok(int),
    }
}

#[derive(Debug, PartialEq)]
struct Challenge {
    question: String,
    answer: u32,
}

fn pick_challenge() -> Challenge {
    Challenge {
        question: String::from("He"),
        answer: 2,
    }
}

fn ask_challenge(challenge: Challenge) -> Result<(), String> {
    println!("Atomic number for {}?", challenge.question);
    let input = read_line()?;
    let user_answer = parse_number(String::from(input.trim()))?;
    match check::check_guess(challenge.answer, user_answer) {
        check::Check::Correct => println!("Good answer!"),
        check::Check::Incorrect => println!("Bad answer!"),
    };
    Ok(())
}

fn main_result() -> Result<(), String> {
    let challenge = pick_challenge();
    ask_challenge(challenge)
}

fn main() {
    match main_result() {
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
        Ok(()) => {}
    }
}
