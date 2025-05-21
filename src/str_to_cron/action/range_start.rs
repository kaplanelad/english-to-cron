//! Module for processing range start-related tokens in cron expressions.

use super::super::{action::Kind, cron::Cron, stack::Stack};
use regex::Regex;
use std::sync::LazyLock;

/// Regular expression to match keywords indicating the start of a range (e.g., "between", "starting").
static RE_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(between|starting|start)").unwrap());

/// Regular expression to specifically match "between".
static RE_MATCH_BETWEEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)(between)").unwrap());

/// Checks if the provided token matches range start-related keywords.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the cron object to interpret range start-related tokens.
pub fn process(token: &str, cron: &mut Cron) {
    let mut stack = Stack::builder(Kind::RangeStart).build();

    // Detect if this is a "between" range
    stack.is_between_range = RE_MATCH_BETWEEN.is_match(token);

    cron.stack.push(stack);
}
