use crate::error::PeriodError;
use chrono::{Local, Months};

use super::validate_non_negative;
use crate::relative::types::Relative;

/// Returns a [`Relative`] moment `years` calendar years in the past.
///
/// Internally converts years to months. A value of `0` returns the current
/// date-time. Use `.as_date()` to get a [`chrono::NaiveDate`] if you do not need the
/// time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `years` is negative.
/// Returns [`PeriodError::Overflow`] if the equivalent month count overflows or the resulting date-time is out of range.
/// Use [`years_from_now`] for future offsets.
#[inline]
pub fn years_ago(years: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(years, "years", "years_from_now")?;
    let months_i64 = years.checked_mul(12).ok_or(PeriodError::Overflow {
        unit: "years",
        value: years,
    })?;
    let months = u32::try_from(months_i64).map_err(|_| PeriodError::Overflow {
        unit: "years",
        value: years,
    })?;
    Local::now()
        .checked_sub_months(Months::new(months))
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "years",
            value: years,
        })
}

/// Returns a [`Relative`] moment `years` calendar years in the future.
///
/// Internally converts years to months. A value of `0` returns the current
/// date-time. Use `.as_date()` to get a [`chrono::NaiveDate`] if you do not need the
/// time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `years` is negative.
/// Returns [`PeriodError::Overflow`] if the equivalent month count overflows or the resulting date-time is out of range.
/// Use [`years_ago`] for past offsets.
#[inline]
pub fn years_from_now(years: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(years, "years", "years_ago")?;
    let months_i64 = years.checked_mul(12).ok_or(PeriodError::Overflow {
        unit: "years",
        value: years,
    })?;
    let months = u32::try_from(months_i64).map_err(|_| PeriodError::Overflow {
        unit: "years",
        value: years,
    })?;
    Local::now()
        .checked_add_months(Months::new(months))
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "years",
            value: years,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PeriodError;
    use chrono::{Local, Months};

    #[test]
    fn test_years_ago_returns_correct_date() {
        let date = years_ago(2).unwrap().as_date();
        let expected = crate::relative::functions::month::months_ago(24)
            .unwrap()
            .as_date();
        assert_eq!(date, expected);
    }

    #[test]
    fn test_years_ago_with_zero_returns_today() {
        assert_eq!(years_ago(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_years_ago_returns_correct_datetime() {
        let lower = Local::now().checked_sub_months(Months::new(24)).unwrap();
        let result = years_ago(2).unwrap().as_datetime();
        let upper = Local::now().checked_sub_months(Months::new(24)).unwrap();
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_years_ago_negative_returns_error() {
        assert_eq!(
            years_ago(-2).unwrap_err().to_string(),
            "years must be positive. Did you mean years_from_now(2)?"
        );
    }

    #[test]
    fn test_years_ago_overflow_returns_error() {
        let result = years_ago(1_000_000_000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "years value 1000000000 is too large"
        );
    }

    #[test]
    fn test_years_from_now_returns_correct_date() {
        let date = years_from_now(2).unwrap().as_date();
        let expected = crate::relative::functions::month::months_from_now(24)
            .unwrap()
            .as_date();
        assert_eq!(date, expected);
    }

    #[test]
    fn test_years_from_now_with_zero_returns_today() {
        assert_eq!(
            years_from_now(0).unwrap().as_date(),
            Local::now().date_naive()
        );
    }

    #[test]
    fn test_years_from_now_returns_correct_datetime() {
        let lower = Local::now().checked_add_months(Months::new(24)).unwrap();
        let result = years_from_now(2).unwrap().as_datetime();
        let upper = Local::now().checked_add_months(Months::new(24)).unwrap();
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_years_from_now_negative_returns_error() {
        assert_eq!(
            years_from_now(-2).unwrap_err().to_string(),
            "years must be positive. Did you mean years_ago(2)?"
        );
    }

    #[test]
    fn test_years_from_now_overflow_returns_error() {
        let result = years_from_now(1_000_000_000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "years value 1000000000 is too large"
        );
    }

    #[test]
    fn test_years_ago_negative_one() {
        let err = years_ago(-1).unwrap_err();
        assert!(matches!(
            err,
            PeriodError::NegativeValue {
                unit: "years",
                suggestion: "years_from_now",
                value: 1,
            }
        ));
    }

    #[test]
    fn test_years_ago_is_in_the_past() {
        assert!(years_ago(1).unwrap().as_date() < Local::now().date_naive());
    }

    #[test]
    fn test_years_from_now_is_in_the_future() {
        assert!(years_from_now(1).unwrap().as_date() > Local::now().date_naive());
    }
}
