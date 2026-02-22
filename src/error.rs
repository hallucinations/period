use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TempusError {
    NegativeValue { unit: &'static str, suggestion: &'static str, value: i64 },
    Overflow { unit: &'static str, value: i64 },
}

impl std::error::Error for TempusError {}

impl fmt::Display for TempusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TempusError::NegativeValue { unit, suggestion, value } => {
                write!(
                    f,
                    "{} must be positive. Did you mean {}({})?",
                    unit, suggestion, value
                )
            }
            TempusError::Overflow { unit, value } => {
                write!(f, "{} value {} is too large", unit, value)
            }
        }
    }
}
