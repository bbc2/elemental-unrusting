extern crate im;

mod check;
pub mod challenge;
use self::im::*;

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    InChallenge(list::List<challenge::Challenge>),
}

pub fn initial(challenges: list::List<challenge::Challenge>) -> State {
    State::InChallenge(challenges)
}

pub fn prompt(state: State) -> String {
    match state {
        State::InChallenge(challenges) => {
            match challenges.uncons() {
                None => {
                    unreachable!();
                },
                Some((challenge, _)) => {
                    format!(
                        "Atomic number for {}?",
                        challenge.question
                    )
                },
            }
        }
    }
}

pub fn end(state: State) -> bool {
    match state {
        State::InChallenge(questions) => {
            questions.is_empty()
        },
    }
}

fn parse_number(input: String) -> Result<u32, String> {
    match input.parse::<u32>() {
        Err(error) => Err(format!("{}", error)),
        Ok(int) => Ok(int),
    }
}

pub fn next(state: State, input: String) -> (State, String) {
    match state {
        State::InChallenge(challenges) =>
            match parse_number(input) {
                Ok(guess) => {
                    match challenges.uncons() {
                        None => {
                            unreachable!();
                        },
                        Some((challenge, remaining)) => {
                            match check::check_guess(challenge.answer, guess) {
                                check::Check::Correct => {
                                    (State::InChallenge(remaining), String::from("Good answer!"))
                                },
                                check::Check::Incorrect => {
                                    (State::InChallenge(challenges), String::from("Bad answer!"))
                                },
                            }
                        },
                    }
                },
                Err(message) => {
                    (State::InChallenge(challenges), message)
                },
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial() {
        let challenges = list![
            challenge::Challenge{question: String::from("He"), answer: 2}
        ];

        let result = initial(challenges.clone());

        let expected = State::InChallenge(challenges);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_prompt() {
        let challenges = list![
            challenge::Challenge{question: String::from("H"), answer: 1},
            challenge::Challenge{question: String::from("He"), answer: 2}
        ];

        let result = prompt(State::InChallenge(challenges));

        assert_eq!("Atomic number for H?", result);
    }

    #[test]
    fn test_next_challenge_good() {
        let challenges = list![
            challenge::Challenge{question: String::from("H"), answer: 1},
            challenge::Challenge{question: String::from("He"), answer: 2}
        ];

        let result = next(State::InChallenge(challenges), String::from("1"));

        let remaining = list![
            challenge::Challenge{question: String::from("He"), answer: 2}
        ];
        let expected = (State::InChallenge(remaining), String::from("Good answer!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_bad() {
        let challenges = list![
            challenge::Challenge{question: String::from("H"), answer: 1},
            challenge::Challenge{question: String::from("He"), answer: 2}
        ];

        let result = next(State::InChallenge(challenges.clone()), String::from("3"));

        let expected = (State::InChallenge(challenges), String::from("Bad answer!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_invalid() {
        let challenges = list![
            challenge::Challenge{question: String::from("H"), answer: 1}
        ];

        let result = next(State::InChallenge(challenges.clone()), String::from(""));

        let expected = (
            State::InChallenge(challenges),
            String::from("cannot parse integer from empty string"),
        );
        assert_eq!(expected, result);
    }
}
