//! Module for processing month tokens in cron expressions.
//!
//! This module provides functionality to interpret and process month-related tokens
//! in the context of cron scheduling. It defines regular expressions for matching
//! various month formats and provides functions for token validation and processing.

use super::super::{
    action::Kind,
    cron::Cron,
    stack::{Stack, StartEndString},
    Error, Result,
};
use regex::Regex;
use std::fmt::Write;
use std::sync::LazyLock;

/// Regular expression to match valid month input in various formats (e.g., "January", "JAN").
static RE_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^((months|month)|(((january|february|march|april|may|june|july|august|september|october|november|december|JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEPT|OCT|NOV|DEC)( ?and)?,? ?)+))$").unwrap()
});

/// Regular expression to match the word "month" or "months".
static RE_MONTH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^(month|months)$").unwrap());

/// Regular expression to find month abbreviations in the input string.
static RE_MONTHS_ABBREVIATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC)").unwrap());

const MONTHS: [&str; 12] = [
    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
];

/// Checks if the provided token is a valid month representation.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the given month token and updates the cron structure accordingly.
///
/// This function interprets the month token and modifies the `cron` object to
/// reflect the corresponding month settings. It handles various scenarios, such as
/// frequency specifications, ranges, and default settings.
///
/// # Returns
/// A [`Result<()>`] indicating success or failure. In case of an incorrect month format,
/// an `Error::IncorrectValue` is returned.
///
pub fn process(token: &str, cron: &mut Cron) -> Result<()> {
    if RE_MONTH.is_match(token) {
        if let Some(element) = cron.stack.last() {
            if element.owner == Kind::FrequencyOnly {
                // cron.syntax.month = format!("0/{}", element.frequency_to_string());
                cron.syntax.month = element.frequency_to_string();
                cron.stack.pop();
            } else if element.owner == Kind::FrequencyWith {
                cron.syntax.month = element.frequency_to_string();
                cron.stack.pop();
            } else if element.owner == Kind::RangeEnd {
                cron.syntax.day_of_month = format!(
                    "{},{}",
                    element.frequency_start.unwrap_or_default(),
                    element.frequency_end.unwrap_or_default()
                );
            } else {
                cron.syntax.month = "*".to_string();
            }
        } else {
            cron.syntax.month = "*".to_string();
        }
    } else {
        let matches: Vec<_> = RE_MONTHS_ABBREVIATION.find_iter(token).collect();
        if matches.is_empty() {
            return Err(Error::IncorrectValue {
                state: "month".to_string(),
                error: format!("value {token} is not a month format"),
            });
        }

        cron.syntax.month = String::new();

        let months: Vec<String> = matches
            .iter()
            .map(|month| month.as_str().to_uppercase())
            .collect::<Vec<_>>();

        if let Some(element) = cron.stack.last_mut() {
            if element.owner == Kind::FrequencyOnly || element.owner == Kind::FrequencyWith {
                cron.syntax.day_of_month = element.frequency_to_string();
                cron.stack.pop();
            } else if element.owner == Kind::RangeStart {
                element.month = Some(element.month.as_ref().map_or_else(
                    || StartEndString {
                        start: months.first().cloned(),
                        end: None,
                    },
                    |month| StartEndString {
                        start: months.first().cloned(),
                        end: month.end.clone(),
                    },
                ));
                cron.stack.pop();
                return Ok(());
            } else if element.owner == Kind::RangeEnd {
                if let Some(frequency_end) = element.frequency_end {
                    cron.syntax.day_of_week = "?".to_string();
                    if let Some(frequency_start) = element.frequency_start {
                        cron.syntax.day_of_month = format!("{frequency_start}-{frequency_end}");
                    }
                }

                let data = element.month.as_ref().map_or_else(
                    || StartEndString {
                        start: None,
                        end: months.first().cloned(),
                    },
                    |month| StartEndString {
                        start: month.start.clone(),
                        end: months.first().cloned(),
                    },
                );
                element.month = Some(data.clone());

                if let (Some(start), Some(end)) = (data.start, data.end) {
                    cron.syntax.month = format!("{start}-{end}");
                }

                cron.stack.pop();
                return Ok(());
            } else {
                cron.stack.pop();
            }
        }

        for &month in &MONTHS {
            if months.contains(&month.to_string()) && !cron.syntax.month.contains(month) {
                write!(cron.syntax.month, "{month},").unwrap();
            }
        }

        cron.syntax.month = cron.syntax.month.trim_end_matches(',').to_string();
    }

    cron.stack.push(
        Stack::builder(Kind::Month)
            .month(StartEndString {
                start: Some(cron.syntax.month.clone()),
                end: None,
            })
            .build(),
    );

    Ok(())
}
