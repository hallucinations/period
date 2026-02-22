use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TempusError {
    NegativeValue { unit: String, suggestion: String, value: i64 },
}

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
        }
    }
}
