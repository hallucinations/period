use crate::error::PeriodError;
use chrono::{Duration, Local, NaiveDate};

use super::validate_non_negative;
use crate::relative::types::Relative;

/// Returns a [`Relative`] moment `days` days in the past.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `days` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`days_from_now`] for future offsets.
#[inline]
pub fn days_ago(days: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(days, "days", "days_from_now")?;
    let duration = Duration::try_days(days).ok_or(PeriodError::Overflow {
        unit: "days",
        value: days,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "days",
            value: days,
        })
}

/// Returns a [`Relative`] moment `days` days in the future.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `days` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`days_ago`] for past offsets.
#[inline]
pub fn days_from_now(days: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(days, "days", "days_ago")?;
    let duration = Duration::try_days(days).ok_or(PeriodError::Overflow {
        unit: "days",
        value: days,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "days",
            value: days,
        })
}

/// Returns yesterday's local date.
///
/// # Panics
///
/// Panics if today is [`NaiveDate::MIN`], which cannot occur in practice.
#[must_use]
#[inline]
pub fn yesterday() -> NaiveDate {
    Local::now()
        .date_naive()
        .pred_opt()
        .expect("date underflow")
}

/// Returns tomorrow's local date.
///
/// # Panics
///
/// Panics if today is [`NaiveDate::MAX`], which cannot occur in practice.
#[must_use]
#[inline]
pub fn tomorrow() -> NaiveDate {
    Local::now().date_naive().succ_opt().expect("date overflow")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PeriodError;
    use chrono::{Duration, Local};

    #[test]
    fn test_days_ago_returns_correct_date() {
        let date = days_ago(3).unwrap().as_date();
        let expected = Local::now().date_naive() - Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_ago_with_zero_returns_today() {
        assert_eq!(days_ago(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_days_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::days(3);
        let result = days_ago(3).unwrap().as_datetime();
        let upper = Local::now() - Duration::days(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_days_ago_negative_returns_error() {
        assert_eq!(
            days_ago(-3).unwrap_err().to_string(),
            "days must be positive. Did you mean days_from_now(3)?"
        );
    }

    #[test]
    fn test_days_ago_overflow_returns_error() {
        // chrono's NaiveDate range is ±~262,000 years (~95,000,000 days);
        // 200_000_000 days exceeds that limit and must overflow.
        assert!(days_ago(200_000_000).is_err());
    }

    #[test]
    fn test_days_from_now_returns_correct_date() {
        let date = days_from_now(3).unwrap().as_date();
        let expected = Local::now().date_naive() + Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_from_now_with_zero_returns_today() {
        assert_eq!(
            days_from_now(0).unwrap().as_date(),
            Local::now().date_naive()
        );
    }

    #[test]
    fn test_days_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::days(3);
        let result = days_from_now(3).unwrap().as_datetime();
        let upper = Local::now() + Duration::days(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_days_from_now_negative_returns_error() {
        assert_eq!(
            days_from_now(-3).unwrap_err().to_string(),
            "days must be positive. Did you mean days_ago(3)?"
        );
    }

    #[test]
    fn test_days_from_now_overflow_returns_error() {
        // chrono's NaiveDate range is ±~262,000 years (~95,000,000 days);
        // 200_000_000 days exceeds that limit and must overflow.
        assert!(days_from_now(200_000_000).is_err());
    }

    #[test]
    fn test_yesterday_returns_previous_date() {
        let expected = Local::now().date_naive().pred_opt().unwrap();
        assert_eq!(yesterday(), expected);
    }

    #[test]
    fn test_tomorrow_returns_next_date() {
        let expected = Local::now().date_naive().succ_opt().unwrap();
        assert_eq!(tomorrow(), expected);
    }

    #[test]
    fn test_negative_value_error_has_correct_value_field() {
        let err = days_ago(-5).unwrap_err();
        assert!(
            matches!(err, PeriodError::NegativeValue { value: 5, .. }),
            "expected value=5, got {err:?}"
        );
    }

    #[test]
    fn test_period_error_equality() {
        assert_eq!(days_ago(-1).unwrap_err(), days_ago(-1).unwrap_err());
    }

    #[test]
    fn test_period_error_implements_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(days_ago(-1).unwrap_err());
        assert!(err.source().is_none());
    }

    #[test]
    fn test_days_ago_plus_n_equals_today() {
        let n = 5i64;
        let date = days_ago(n).unwrap().as_date();
        assert_eq!(date + Duration::days(n), Local::now().date_naive());
    }

    #[test]
    fn test_days_from_now_minus_n_equals_today() {
        let n = 5i64;
        let date = days_from_now(n).unwrap().as_date();
        assert_eq!(date - Duration::days(n), Local::now().date_naive());
    }

    #[test]
    fn test_days_ago_large_valid_value() {
        assert!(days_ago(365).is_ok());
    }

    #[test]
    fn test_yesterday_and_tomorrow_are_two_days_apart() {
        assert_eq!(tomorrow() - yesterday(), Duration::days(2));
    }

    #[test]
    fn test_yesterday_is_before_today() {
        assert!(yesterday() < Local::now().date_naive());
    }

    #[test]
    fn test_tomorrow_is_after_today() {
        assert!(tomorrow() > Local::now().date_naive());
    }

    #[test]
    fn test_yesterday_equals_days_ago_1_date() {
        assert_eq!(yesterday(), days_ago(1).unwrap().as_date());
    }

    #[test]
    fn test_tomorrow_equals_days_from_now_1_date() {
        assert_eq!(tomorrow(), days_from_now(1).unwrap().as_date());
    }
}
