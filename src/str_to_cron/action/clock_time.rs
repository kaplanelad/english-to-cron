//! This file provides functionality for processing clock time tokens, converting them into the
//! appropriate format for cron syntax. It recognizes various time formats, including 12-hour
//! format with AM/PM and 24-hour format, as well as specific keywords like "noon" and "midnight".
//!
//! The regex patterns defined here help to match and extract hours and minutes from the tokens.
//!
//! This is part of a broader module that converts human-readable strings into cron syntax.

use super::super::{
    action::Kind,
    cron::Cron,
    stack::{Stack, StartEnd},
    Error, Result,
};

use regex::Regex;
use std::sync::LazyLock;

/// A regex pattern that matches various clock time formats, including:
/// - 12-hour format with AM/PM (e.g., "5 PM", "7 AM")
/// - 24-hour format (e.g., "13:00")
/// - Special cases for "noon" and "midnight"
static RE_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^([0-9]+:)?[0-9]+ *(AM|PM)$|^([0-9]+:[0-9]+)$|(noon|midnight)").unwrap()
});

/// A regex pattern to extract the hour from a time token.
static RE_HOUR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+").unwrap());

/// A regex pattern to extract the minute from a time token.
static RE_MINUTE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r":[0-9]+").unwrap());

/// A regex pattern that matches the keywords "noon" and "midnight".
static RE_NOON_MIDNIGHT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(noon|midnight)").unwrap());

/// Checks if a given string token matches the expected clock time format.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

#[allow(clippy::too_many_lines)]
/// Processes a clock time token and updates the corresponding fields in the cron syntax structure.
///
/// This function extracts hours and minutes from the token, handles conversions from 12-hour to 24-hour format,
/// and sets the appropriate values in the `Cron` struct. It also handles specific cases for "noon" and "midnight".
///
/// # Errors
///
/// Returns an error if parsing the hour or minute fails, if values are out of range, or if the time is incorrect.
pub fn process(token: &str, cron: &mut Cron) -> Result<()> {
    let mut hour = 0;
    let mut minute = 0;

    if let Some(hour_str) = RE_HOUR.find(token) {
        hour = hour_str
            .as_str()
            .parse::<i32>()
            .map_err(|_| Error::ParseToNumber {
                state: "clock_time".to_string(),
                value: hour_str.as_str().to_string(),
            })?;
    }

    if let Some(minute_str) = RE_MINUTE.find(token) {
        if minute_str.as_str().contains(':') {
            if let Some(minute_str) = minute_str.as_str().split(':').nth(1) {
                minute = minute_str
                    .parse::<i32>()
                    .map_err(|_| Error::ParseToNumber {
                        state: "clock_time".to_string(),
                        value: minute_str.to_string(),
                    })?;
                if minute >= 60 {
                    return Err(Error::IncorrectValue {
                        state: "clock_time".to_string(),
                        error: format!("minute {minute} should be lower or equal to 60"),
                    });
                }
            }
        }
    }

    match token.to_lowercase().as_str() {
        _ if token.to_lowercase().contains("pm") => {
            match hour.cmp(&12) {
                std::cmp::Ordering::Less => hour += 12,
                std::cmp::Ordering::Greater => {
                    return Err(Error::IncorrectValue {
                        state: "clock_time".to_string(),
                        error: format!("please correct the time before PM. value: {hour}"),
                    });
                }
                std::cmp::Ordering::Equal => {} // Do nothing, hour remains 12
            }
        }
        _ if token.to_lowercase().contains("am") => {
            match hour.cmp(&12) {
                std::cmp::Ordering::Equal => hour = 0,
                std::cmp::Ordering::Greater => {
                    return Err(Error::IncorrectValue {
                        state: "clock_time".to_string(),
                        error: format!("please correct the time before AM. value: {hour}"),
                    });
                }
                std::cmp::Ordering::Less => {} // Do nothing, hour remains unchanged
            }
        }
        _ => {} // Handle other cases if necessary
    }

    if RE_NOON_MIDNIGHT.is_match(token) {
        if token == "noon" {
            hour = 12;
        } else {
            hour = 0;
        }
        minute = 0;
    }

    if let Some(element) = cron.stack.last_mut() {
        if element.owner == Kind::RangeStart {
            element.hour = Some(StartEnd {
                start: Some(hour),
                end: None,
            });
            return Ok(());
        } else if element.owner == Kind::RangeEnd {
            if let Some(element_hour) = &element.hour {
                if element_hour.start == Some(hour) {
                    element.min = Some(StartEnd {
                        start: Some(hour),
                        end: Some(hour),
                    });
                    cron.syntax.hour = format!("{hour}-{hour}");
                } else {
                    element.hour.clone().unwrap().end = Some(hour);
                    if element.is_and_connector && !element.is_between_range {
                        // Use comma for "and" connector but not in a "between X and Y" context
                        cron.syntax.hour =
                            format!("{},{}", element_hour.start.unwrap_or_default(), hour);
                    } else {
                        // Use hyphen for other range connectors or for "between X and Y"
                        cron.syntax.hour =
                            format!("{}-{}", element_hour.start.unwrap_or_default(), hour);
                    }
                }
            }

            return Ok(());
        }
    }

    cron.syntax.min = minute.to_string();
    cron.syntax.hour = hour.to_string();

    cron.stack.push(
        Stack::builder(Kind::ClockTime)
            .hour(StartEnd {
                start: Some(hour),
                end: None,
            })
            .min(StartEnd {
                start: Some(minute),
                end: None,
            })
            .build(),
    );

    Ok(())
}
