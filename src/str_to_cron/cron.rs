use crate::str_to_cron::Tokenizer;
use std::str::FromStr;

use super::{action, stack::Stack, Error, Result};

#[derive(Default, Debug)]
pub struct Cron {
    pub syntax: Syntax,
    pub stack: Vec<Stack>,
}

#[derive(Debug)]
pub struct Syntax {
    pub seconds: String,
    pub min: String,
    pub hour: String,
    pub day_of_month: String,
    pub day_of_week: String,
    pub month: String,
    pub year: String,
}

impl Default for Syntax {
    fn default() -> Self {
        Self {
            seconds: "0".to_string(),
            min: "*".to_string(),
            hour: "*".to_string(),
            day_of_month: "*".to_string(),
            day_of_week: "?".to_string(),
            month: "*".to_string(),
            year: "*".to_string(),
        }
    }
}

impl std::fmt::Display for Cron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.syntax.seconds.trim(),
            self.syntax.min.trim(),
            self.syntax.hour.trim(),
            self.syntax.day_of_month.trim(),
            self.syntax.month.trim(),
            self.syntax.day_of_week.trim(),
            self.syntax.year.trim(),
        )
    }
}

impl Cron {
    /// Creates a new `Cron` instance from a given cron expression string.
    ///
    /// This function tokenizes the input string and processes each token to construct
    /// a valid `Cron` representation. If the input is empty or contains invalid tokens,
    /// an error is returned.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] if the input is empty or contains invalid tokens.
    ///
    pub fn new(text: &str) -> Result<Self> {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.run(text);

        if tokens.is_empty() {
            return Err(Error::InvalidInput);
        }

        let mut cron = Self::default();
        for token in tokens {
            if let Some(state) = action::try_from_token(&token) {
                state.process(&token, &mut cron)?;
            }
        }
        Ok(cron)
    }
}

impl FromStr for Cron {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
