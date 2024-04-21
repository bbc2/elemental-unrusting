pub mod challenge;

fn normalize(input: &str) -> String {
    return String::from(input.trim());
}

pub struct Game {
    challenges: Vec<challenge::Challenge>,
    failures: u64,
}

impl Game {
    pub fn initial(challenges: &[challenge::Challenge]) -> Self {
        let mut challenges = challenges.to_vec();
        challenges.reverse();
        Self {
            challenges,
            failures: 0,
        }
    }

    fn goto_next(&mut self) {
        self.challenges.pop();
        self.failures = 0;
    }

    pub fn prompt(&self) -> String {
        match self.challenges.last() {
            None => {
                unreachable!()
            }
            Some(challenge) => format!("Atomic number for {}?", challenge.question),
        }
    }

    pub fn next(&mut self, input: &str) -> String {
        let guess = normalize(input);
        match self.challenges.last() {
            None => {
                unreachable!();
            }
            Some(challenge) => {
                if challenge.check(guess) {
                    self.goto_next();
                    String::from("Good answer!")
                } else {
                    let result = String::from("Bad answer!");
                    self.failures += 1;
                    if self.failures > 2 {
                        self.goto_next();
                    }
                    result
                }
            }
        }
    }

    pub fn end(&self) -> bool {
        self.challenges.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt() {
        let challenges = vec![
            challenge::Challenge {
                question: String::from("H"),
                answer: String::from("1"),
            },
            challenge::Challenge {
                question: String::from("He"),
                answer: String::from("2"),
            },
        ];
        let game = Game::initial(&challenges);

        let result = game.prompt();

        assert_eq!("Atomic number for H?", result);
    }

    #[test]
    fn test_next_challenge_good() {
        let challenges = vec![
            challenge::Challenge {
                question: String::from("H"),
                answer: String::from("1"),
            },
            challenge::Challenge {
                question: String::from("He"),
                answer: String::from("2"),
            },
        ];
        let mut game = Game::initial(&challenges);

        let result = game.next("1");

        let expected = String::from("Good answer!");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_challenge_bad() {
        let challenges = vec![
            challenge::Challenge {
                question: String::from("H"),
                answer: String::from("1"),
            },
            challenge::Challenge {
                question: String::from("He"),
                answer: String::from("2"),
            },
        ];
        let mut game = Game::initial(&challenges);

        let result = game.next("3");

        let expected = String::from("Bad answer!");
        assert_eq!(expected, result);
    }
}
