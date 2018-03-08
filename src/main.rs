use std::io;
use std::process;

use std::error::Error;

mod game;

fn read_line() -> Result<String, String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(error) => Err(format!("{}", error.description())),
        Ok(_) => Ok(input),
    }
}

#[derive(Debug, PartialEq)]
struct Challenge {
    question: String,
    answer: u32,
}

fn read_user_line() -> Result<String, String> {
    let input = read_line()?;
    let answer = String::from(input.trim());
    Ok(answer)
}

fn main_result() -> Result<(), String> {
    let mut state = game::initial();
    println!("Welcome to the game");
    while !game::end(state) {
        let input =
            if game::input(state) {
                Some (read_user_line()?)
            } else {
                None
            };
        let (new_state, message) = game::next(state, input);
        state = new_state;
        println!("{}", message);
    }
    Ok(())
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
