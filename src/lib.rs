#[allow(clippy::needless_doctest_main)]
#[allow(clippy::doc_markdown)]
#[doc = include_str!("../README.md")]
mod str_to_cron;
pub use str_to_cron::{Cron, Error, Result};

/// Converts an English description of a schedule into cronjob syntax.
///
/// This function takes a natural language description of a recurring schedule
/// (e.g., "Run every 15 seconds", "Run at 6:00 pm every Monday through Friday")
/// and converts it into a valid cron expression that can be used to schedule jobs.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use english_to_cron::str_cron_syntax;
///
/// assert_eq!(str_cron_syntax("every 15 seconds").unwrap(), "0/15 * * * * ? *");
/// assert_eq!(str_cron_syntax("every minute").unwrap(), "0 * * * * ? *");
/// assert_eq!(str_cron_syntax("every day at 4:00 pm").unwrap(), "0 0 16 */1 * ? *");
/// assert_eq!(str_cron_syntax("at 10:00 am").unwrap(), "0 0 10 * * ? *");
/// assert_eq!(str_cron_syntax("Run at midnight on the 1st and 15th of the month").unwrap(), "0 0 0 1,15 * ? *");
/// assert_eq!(str_cron_syntax("on Sunday at 12:00").unwrap(), "0 0 12 ? * SUN *");
/// ```
///
/// # Errors
///
/// This function returns an [`Error`] if it is unable to parse the provided string
/// into a valid cron syntax. This may occur when the input is incomplete, ambiguous,
/// or does not follow a recognizable pattern.
///
/// # Return
///
/// Returns a [`Result`] containing the parsed cron expression as a `String` on success,
/// or an [`Error`] if parsing fails.
///
/// [`Error`]: str_to_cron::Error
pub fn str_cron_syntax(input: &str) -> str_to_cron::Result<String> {
    let cron = str_to_cron::Cron::new(input)?;
    Ok(format!("{cron}"))
}
