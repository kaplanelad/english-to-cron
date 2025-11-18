//! This module defines error types and handling for the "English to Corn" project.
//!
//! The `Error` enum represents the different kinds of errors that can occur during
//! the processing of input data. Each error variant captures specific details about the error,
//! allowing for more descriptive and accurate error reporting.
//!
//! The module also provides a type alias `Result<T>` for convenience, defaulting to
//! using the `Error` type as the error variant in the `std::result::Result`.

/// Represents the different kinds of errors that can occur in the "English to Corn" project.
///
/// The variants capture specific error scenarios, such as invalid input or failed parsing
/// operations, along with relevant state or context information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Error variant for invalid input.
    /// This variant is used when the input provided is not in a human-readable format.
    InvalidInput,
    /// Error variant for capture-related failures.
    /// This occurs when a specific token cannot be captured within a given state.
    ///
    /// # Fields
    /// - `state`: The state in which the error occurred.
    /// - `token`: The token that could not be captured.
    Capture { state: String, token: String },
    /// Error variant for failed parsing to a number.
    /// This occurs when a value could not be parsed as a number within a specific state.
    ///
    /// # Fields
    /// - `state`: The state in which the error occurred.
    /// - `value`: The value that could not be parsed into a number.
    ParseToNumber { state: String, value: String },
    /// Error variant for incorrect or invalid values.
    /// This is triggered when an invalid value is encountered in a given state.
    ///
    /// # Fields
    /// - `state`: The state in which the error occurred.
    /// - `error`: A description of the error or the reason why the value is considered invalid.
    IncorrectValue { state: String, error: String },
}

/// Implements the `Display` trait for the `Error` enum.
///
/// This allows for user-friendly error messages to be printed, making it easier
/// to understand the cause of an error when it occurs.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Please enter human readable"),
            Self::Capture { state, token } => {
                write!(f, "Could not capture: {token} in state: {state} ")
            }
            Self::ParseToNumber { state, value } => {
                write!(f, "Could not parse: {value} to number. state: {state} ")
            }
            Self::IncorrectValue { state, error } => {
                write!(
                    f,
                    "value is invalid in state: {state}. description: {error} "
                )
            }
        }
    }
}

/// Implements the `std::error::Error` trait for the `Error` enum.
///
/// This allows the error type to be used with the `?` operator and error handling
/// libraries like `anyhow` and `thiserror`. The default implementation is sufficient
/// since `Error` already implements `Display` and `Debug`.
impl std::error::Error for Error {}

/// Custom `Result` type alias for the "English to Corn" project.
///
/// This is a convenience alias for `std::result::Result` where the error type defaults to the `Error` enum.
pub type Result<T, E = Error> = std::result::Result<T, E>;
