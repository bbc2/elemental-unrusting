#[derive(Debug, PartialEq)]
pub enum Check {
    Incorrect,
    Correct,
}

pub fn check_guess(expected: u32, actual: u32) -> Check {
    if expected == actual {
        Check::Correct
    } else {
        Check::Incorrect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_guess_incorrect() {
        let result = check_guess(0, 1);

        assert_eq!(Check::Incorrect, result);
    }

    #[test]
    fn test_check_guess_correct() {
        let result = check_guess(0, 0);

        assert_eq!(Check::Correct, result);
    }
}
