/// Module for processing range-related tokens in cron expressions.
///
/// This module handles the interpretation of tokens that represent ranges or connections
/// between elements in cron scheduling, such as "to", "through", "ending", and "and".
use super::super::{action::Kind, cron::Cron, stack::StartEndString};
use regex::Regex;
use std::sync::LazyLock;

/// Regular expression to match range-related keywords (e.g., "to", "through").
static RE_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(to|through|ending|end|and)").unwrap());

/// Regular expression to specifically match "and".
static RE_MATCH_AND: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)(and)").unwrap());

/// Checks if the provided token matches range-related keywords.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the cron object to interpret range-related tokens.
pub fn process(token: &str, cron: &mut Cron) {
    // Check if the token is "and" specifically
    let is_and = RE_MATCH_AND.is_match(token);

    if let Some(element) = cron.stack.last_mut() {
        // Set the is_and flag in the element so we know to use comma instead of hyphen
        element.is_and_connector = is_and;

        match element.owner {
            Kind::FrequencyWith | Kind::FrequencyOnly => {
                element.frequency_start = element.frequency;
            }

            Kind::Day => {
                element.day = match &element.day {
                    Some(day) => Some(StartEndString {
                        start: element.day_of_week.clone(),
                        end: day.end.clone(),
                    }),
                    None => Some(StartEndString {
                        start: element.day_of_week.clone(),
                        end: None,
                    }),
                };
            }

            Kind::Month => {
                element.owner = Kind::RangeEnd;
            }

            Kind::RangeStart => element.owner = Kind::RangeEnd,
            Kind::Year
            | Kind::ClockTime
            | Kind::Minute
            | Kind::Hour
            | Kind::RangeEnd
            | Kind::Secund
            | Kind::OnlyOn => {}
        }
        element.owner = Kind::RangeEnd;
    }
}
