extern crate im;

pub mod challenge;
use self::im::*;

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    InChallenge(vector::Vector<challenge::Challenge>),
}

pub fn initial(challenges: vector::Vector<challenge::Challenge>) -> State {
    State::InChallenge(challenges)
}

pub fn prompt(state: State) -> String {
    match state {
        State::InChallenge(challenges) => match challenges.uncons() {
            None => {
                unreachable!();
            }
            Some((challenge, _)) => format!("Atomic number for {}?", challenge.question),
        },
    }
}

pub fn end(state: State) -> bool {
    match state {
        State::InChallenge(questions) => questions.is_empty(),
    }
}

fn normalize(input: String) -> String {
    return String::from(input.trim());
}

pub fn next(state: State, input: String) -> (State, String) {
    match state {
        State::InChallenge(challenges) => {
            let guess = normalize(input);
            match challenges.uncons() {
                None => {
                    unreachable!();
                }
                Some((challenge, remaining)) => {
                    if challenge.check(guess) {
                        (State::InChallenge(remaining), String::from("Good answer!"))
                    } else {
                        (State::InChallenge(challenges), String::from("Bad answer!"))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial() {
        let challenges = vector![challenge::Challenge {
            question: String::from("He"),
            answer: String::from("2")
        }];

        let result = initial(challenges.clone());

        let expected = State::InChallenge(challenges);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_prompt() {
        let challenges = vector![
            challenge::Challenge {
                question: String::from("H"),
                answer: String::from("1")
            },
            challenge::Challenge {
                question: String::from("He"),
                answer: String::from("2")
            }
        ];

        let result = prompt(State::InChallenge(challenges));

        assert_eq!("Atomic number for H?", result);
    }

    #[test]
    fn test_next_challenge_good() {
        let challenges = vector![
            challenge::Challenge {
                question: String::from("H"),
                answer: String::from("1")
            },
            challenge::Challenge {
                question: String::from("He"),
                answer: String::from("2")
            }
        ];

        let result = next(
            State::InChallenge(challenges),
            String::from(String::from("1")),
        );

        let remaining = vector![challenge::Challenge {
            question: String::from("He"),
            answer: String::from("2")
        }];
        let expected = (State::InChallenge(remaining), String::from("Good answer!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_bad() {
        let challenges = vector![
            challenge::Challenge {
                question: String::from("H"),
                answer: String::from("1")
            },
            challenge::Challenge {
                question: String::from("He"),
                answer: String::from("2")
            }
        ];

        let result = next(State::InChallenge(challenges.clone()), String::from("3"));

        let expected = (State::InChallenge(challenges), String::from("Bad answer!"));
        assert_eq!(expected, result);
    }
}
