extern crate im;

mod check;
use self::im::*;

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Challenge(list::List<u32>),
}

pub fn initial(questions: list::List<u32>) -> State {
    State::Challenge(questions)
}

fn atomic_to_symbol(atomic: u32) -> String {
    if atomic == 1 {
        String::from("H")
    } else if atomic == 2 {
        String::from("He")
    } else {
        format!("Atomic<{}>", atomic)
    }
}

pub fn prompt(state: State) -> String {
    match state {
        State::Challenge(questions) => {
            match questions.uncons() {
                None => {
                    unreachable!();
                },
                Some((question, _)) => {
                    format!(
                        "Atomic number for {}?",
                        atomic_to_symbol(*question),
                    )
                },
            }
        }
    }
}

pub fn end(state: State) -> bool {
    match state {
        State::Challenge(questions) => {
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
        State::Challenge(questions) =>
            match parse_number(input) {
                Ok(guess) => {
                    match questions.uncons() {
                        None => {
                            unreachable!();
                        },
                        Some((question, remaining)) => {
                            match check::check_guess(*question, guess) {
                                check::Check::Correct => {
                                    (State::Challenge(remaining), String::from("Good answer!"))
                                },
                                check::Check::Incorrect => {
                                    (State::Challenge(questions), String::from("Bad answer!"))
                                },
                            }
                        },
                    }
                },
                Err(message) => {
                    (State::Challenge(questions), message)
                },
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial() {
        let result = initial(list![1, 2]);

        let expected = State::Challenge(list![1, 2]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_prompt() {
        let result = prompt(State::Challenge(list![1, 2]));

        assert_eq!("Atomic number for H?", result);
    }

    #[test]
    fn test_next_challenge_good() {
        let result = next(State::Challenge(list![1, 2]), String::from("1"));

        let expected = (State::Challenge(list![2]), String::from("Good answer!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_bad() {
        let result = next(State::Challenge(list![1, 2]), String::from("3"));

        let expected = (State::Challenge(list![1, 2]), String::from("Bad answer!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_invalid() {
        let result = next(State::Challenge(list![1]), String::from(""));

        let expected = (
            State::Challenge(list![1]),
            String::from("cannot parse integer from empty string"),
        );
        assert_eq!(expected, result);
    }
}
