mod check;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Initial,
    Challenge,
    Final,
}

pub fn initial() -> State {
    State::Initial
}

pub fn input(state: State) -> bool {
    match state {
        State::Initial => false,
        State::Challenge => true,
        State::Final => false,
    }
}

pub fn end(state: State) -> bool {
    match state {
        State::Initial => false,
        State::Challenge => false,
        State::Final => true,
    }
}

fn parse_number(input: String) -> Result<u32, String> {
    match input.parse::<u32>() {
        Err(error) => Err(format!("{}", error)),
        Ok(int) => Ok(int),
    }
}

pub fn next(state: State, input: Option<String>) -> (State, String) {
    match state {
        State::Initial =>
            (State::Challenge, String::from("Atomic number for He?")),
        State::Challenge =>
            match input {
                Some(input) => {
                    match parse_number(input) {
                        Ok(guess) => {
                            match check::check_guess(2, guess) {
                                check::Check::Correct => {
                                    (State::Final, String::from("Good answer!"))
                                },
                                check::Check::Incorrect => {
                                    (State::Final, String::from("Bad answer!"))
                                },
                            }
                        },
                        Err(message) => {
                            (State::Challenge, message)
                        },
                    }
                },
                None => {
                    unreachable!();
                },
            },
        State::Final =>
            (State::Initial, String::from("Thanks for playing!")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_initial() {
        let result = next(State::Initial, None);

        let expected = (State::Challenge, String::from("Atomic number for He?"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_good() {
        let result = next(State::Challenge, Some (String::from("2")));

        let expected = (State::Final, String::from("Good answer!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_bad() {
        let result = next(State::Challenge, Some (String::from("1")));

        let expected = (State::Final, String::from("Bad answer!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_invalid() {
        let result = next(State::Challenge, Some (String::from("")));

        let expected = (
            State::Challenge,
            String::from("cannot parse integer from empty string"),
        );
        assert_eq!(expected, result);
    }
}
