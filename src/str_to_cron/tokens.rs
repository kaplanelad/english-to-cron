use regex::Regex;
use std::sync::LazyLock;

static RE_TOKENS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:seconds|second|secs|sec)|(?:hours?|hrs?)|(?:minutes?|mins?|min)|(?:months?|(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|may|jun|jul|aug|sept|oct|nov|dec)(?: ?and)?,? ?)+|[0-9]+(?:th|nd|rd|st)|(?:[0-9]+:)?[0-9]+ ?(?:am|pm)|[0-9]+:[0-9]+|(?:noon|midnight)|(?:days?|(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekend|mon|tue|wed|thu|fri|sat|sun)(?: ?and)?,? ?)+|(?:[0-9]{4}[0-9]*(?: ?and)?,? ?)+|[0-9]+|(?:only on)|(?:to|through|ending|end|and)|(?:between|starting|start)").unwrap()
});

pub struct Tokenizer {
    regex: Regex,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            regex: RE_TOKENS.clone(),
        }
    }

    #[must_use]
    pub fn run(&self, input_string: &str) -> Vec<String> {
        // Preprocess the input to handle special cases
        let processed_input = input_string.replace(", ", " and ");

        // Handle "only on" followed by day names as a special pattern
        let processed_input = if processed_input.contains("only on") {
            // Remove "and" before "only on" to prevent misinterpretation
            processed_input.replace(" and only on", " only on")
        } else {
            processed_input
        };

        let matches = self
            .regex
            .find_iter(&processed_input)
            .map(|m| m.as_str().trim().to_string())
            .collect();

        matches
    }
}
