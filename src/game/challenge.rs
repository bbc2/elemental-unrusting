#[derive(Clone, Debug, PartialEq)]
pub struct Challenge {
    pub question: String,
    pub answer: String,
}

impl Challenge {
    pub fn check(&self, guess: String) -> bool {
        self.answer == guess
    }
}
