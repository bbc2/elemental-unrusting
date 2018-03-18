#[derive(Clone, Debug, PartialEq)]
pub struct Challenge {
    pub question: String,
    pub answer: u32,
}

impl Challenge {
    pub fn check(&self, guess: u32) -> bool {
        self.answer == guess
    }
}
