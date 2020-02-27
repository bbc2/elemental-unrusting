extern crate im;
extern crate rand;
#[macro_use]
extern crate serde_derive;

use rand::seq::SliceRandom;
use std::error::Error;
use std::io;
use std::iter::FromIterator;
use std::process;

use im::*;

mod elements;
mod game;

fn read_line() -> Result<String, String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(error) => Err(format!("{}", error.description())),
        Ok(_) => Ok(input),
    }
}

fn read_user_line() -> Result<String, String> {
    let input = read_line()?;
    let answer = String::from(input.trim());
    Ok(answer)
}

fn element_to_challenge(element: elements::Element) -> game::challenge::Challenge {
    return game::challenge::Challenge {
        question: element.symbol,
        answer: format!("{}", element.atomic_number),
    };
}

fn random_challenges(
    elements: Vector<elements::Element>,
    count: u64,
) -> Vector<game::challenge::Challenge> {
    let mut vector = Vec::from_iter(elements.into_iter());
    let mut rng = rand::thread_rng();
    vector.shuffle(&mut rng);
    vector.truncate(count as usize);
    return Vector::from_iter(vector.into_iter().map(|element| {
        return element_to_challenge(element.clone());
    }));
}

fn main_result() -> Result<(), String> {
    let elements = elements::load();
    let challenges = random_challenges(elements, 3);
    let mut state = game::initial(challenges);
    println!("Welcome to the game");
    while !game::end(state.clone()) {
        println!("{}", game::prompt(state.clone()));
        let (new_state, message) = game::next(state, read_user_line()?);
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
