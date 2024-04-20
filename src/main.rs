extern crate rand;
#[macro_use]
extern crate serde_derive;

use rand::seq::SliceRandom;
use std::io;
use std::iter::FromIterator;
use std::process;

mod elements;
mod game;

fn read_line() -> Result<String, String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(error) => Err(format!("{}", error)),
        Ok(_) => Ok(input),
    }
}

fn read_user_line() -> Result<String, String> {
    let input = read_line()?;
    let answer = String::from(input.trim());
    Ok(answer)
}

fn element_to_challenge(element: elements::Element) -> game::challenge::Challenge {
    game::challenge::Challenge {
        question: element.symbol,
        answer: format!("{}", element.atomic_number),
    }
}

fn random_challenges(
    elements: Vec<elements::Element>,
    count: u64,
) -> Vec<game::challenge::Challenge> {
    let mut vector = Vec::from_iter(elements);
    let mut rng = rand::thread_rng();
    vector.shuffle(&mut rng);
    vector.truncate(count as usize);
    Vec::from_iter(
        vector
            .into_iter()
            .map(|element| element_to_challenge(element.clone())),
    )
}

fn main_result() -> Result<(), String> {
    let elements = elements::load();
    let challenges = random_challenges(elements, 3);
    let mut game = game::Game::initial(&challenges);
    println!("Welcome to the game");
    while !game.end() {
        println!("{}", game.prompt());
        let message = game.next(&read_user_line()?);
        println!("{}", message);
    }
    Ok(())
}

fn main() {
    if let Err(error) = main_result() {
        println!("Error: {}", error);
        process::exit(1);
    }
}
