//! This module defines the various kinds of tokens that can be processed in a cron expression.
//! It provides functions to match and process these tokens accordingly.

use super::{cron::Cron, Error, Result};
mod clock_time;
mod day;
mod frequency_only;
mod frequency_with;
mod hour;
mod minute;
mod month;
mod range_end;
mod range_start;
mod seconds;
mod year;

/// An enumeration of the kinds of tokens that can be processed in a cron expression.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Kind {
    /// Token indicating a frequency with specified intervals.
    FrequencyWith,
    /// Token indicating a frequency without specific intervals.
    FrequencyOnly,
    /// Token indicating a specific time on a clock.
    ClockTime,
    /// Token indicating days of the week.
    Day,
    /// Token indicating secund.
    Secund,
    /// Token indicating minutes.
    Minute,
    /// Token indicating hours.
    Hour,
    /// Token indicating months.
    Month,
    /// Token indicating years.
    Year,
    /// Token indicating the start of a range.
    RangeStart,
    /// Token indicating the end of a range.
    RangeEnd,
    /// Token indicating "only on" directive.
    OnlyOn,
}

/// Attempts to match the provided token to one of the `Kind` enumerations.
/// Returns `Some(Kind)` if a match is found, or `None` if no match exists.
pub fn try_from_token(token: &str) -> Option<Kind> {
    for state_kind in Kind::iterator() {
        let is_match = match state_kind {
            Kind::FrequencyWith => frequency_with::try_from_token(token),
            Kind::FrequencyOnly => frequency_only::try_from_token(token),
            Kind::ClockTime => clock_time::try_from_token(token),
            Kind::Day => day::try_from_token(token),
            Kind::Secund => seconds::try_from_token(token),
            Kind::Minute => minute::try_from_token(token),
            Kind::Hour => hour::try_from_token(token),
            Kind::Month => month::try_from_token(token),
            Kind::Year => year::try_from_token(token),
            Kind::RangeStart => range_start::try_from_token(token),
            Kind::RangeEnd => range_end::try_from_token(token),
            Kind::OnlyOn => token.to_lowercase() == "only on",
        };
        if is_match {
            return Some(state_kind);
        }
    }
    None
}

impl Kind {
    /// Provides an iterator over all possible [`Kind`] values.
    const fn iterator() -> [Self; 12] {
        [
            Self::FrequencyWith,
            Self::FrequencyOnly,
            Self::ClockTime,
            Self::Day,
            Self::Secund,
            Self::Minute,
            Self::Hour,
            Self::Month,
            Self::Year,
            Self::RangeStart,
            Self::RangeEnd,
            Self::OnlyOn,
        ]
    }

    /// Processes the token based on the kind of token.
    /// Each variant has its own processing logic defined in the respective module.
    /// Returns a `Result<()>` indicating success or failure of the operation.
    pub fn process(self, token: &str, cron: &mut Cron) -> Result<()> {
        match self {
            Self::FrequencyWith => frequency_with::process(token, cron)?,
            Self::FrequencyOnly => {
                let frequency = token.parse::<i32>().map_err(|_| Error::ParseToNumber {
                    state: "frequency_only".to_string(),
                    value: token.to_string(),
                })?;

                frequency_only::process(frequency, cron);
            }
            Self::ClockTime => clock_time::process(token, cron)?,
            Self::Day => day::process(token, cron)?,
            Self::Secund => seconds::process(token, cron),
            Self::Minute => minute::process(token, cron),
            Self::Hour => hour::process(token, cron),
            Self::Month => month::process(token, cron)?,
            Self::Year => year::process(token, cron)?,
            Self::RangeStart => range_start::process(token, cron),
            Self::RangeEnd => range_end::process(token, cron),
            Self::OnlyOn => {
                // When "only on" is encountered, we don't need to do anything special
                // The next token should be a day, which will be handled correctly
            }
        }

        Ok(())
    }
}
