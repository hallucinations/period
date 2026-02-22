use crate::error::TempusError;
use chrono::{DateTime, Duration, Local, Months, NaiveDate};

fn validate_positive(
    value: i64,
    unit: &'static str,
    suggestion: &'static str,
) -> Result<(), TempusError> {
    if value < 0 {
        return Err(TempusError::NegativeValue {
            unit,
            suggestion,
            value: value.unsigned_abs(),
        });
    }
    Ok(())
}

/// Returns the local date-time `seconds` seconds in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `seconds` is negative.
/// Use [`seconds_from_now`] for future offsets.
#[inline]
pub fn seconds_ago(seconds: i64) -> Result<DateTime<Local>, TempusError> {
    validate_positive(seconds, "seconds", "seconds_from_now")?;
    Ok(Local::now() - Duration::seconds(seconds))
}

/// Returns the local date-time `seconds` seconds in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `seconds` is negative.
/// Use [`seconds_ago`] for past offsets.
#[inline]
pub fn seconds_from_now(seconds: i64) -> Result<DateTime<Local>, TempusError> {
    validate_positive(seconds, "seconds", "seconds_ago")?;
    Ok(Local::now() + Duration::seconds(seconds))
}

/// Returns the local date-time `minutes` minutes in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `minutes` is negative.
/// Use [`minutes_from_now`] for future offsets.
#[inline]
pub fn minutes_ago(minutes: i64) -> Result<DateTime<Local>, TempusError> {
    validate_positive(minutes, "minutes", "minutes_from_now")?;
    Ok(Local::now() - Duration::minutes(minutes))
}

/// Returns the local date-time `minutes` minutes in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `minutes` is negative.
/// Use [`minutes_ago`] for past offsets.
#[inline]
pub fn minutes_from_now(minutes: i64) -> Result<DateTime<Local>, TempusError> {
    validate_positive(minutes, "minutes", "minutes_ago")?;
    Ok(Local::now() + Duration::minutes(minutes))
}

/// Returns the local date-time `hours` hours in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `hours` is negative.
/// Use [`hours_from_now`] for future offsets.
#[inline]
pub fn hours_ago(hours: i64) -> Result<DateTime<Local>, TempusError> {
    validate_positive(hours, "hours", "hours_from_now")?;
    Ok(Local::now() - Duration::hours(hours))
}

/// Returns the local date-time `hours` hours in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `hours` is negative.
/// Use [`hours_ago`] for past offsets.
#[inline]
pub fn hours_from_now(hours: i64) -> Result<DateTime<Local>, TempusError> {
    validate_positive(hours, "hours", "hours_ago")?;
    Ok(Local::now() + Duration::hours(hours))
}

/// Returns the local date `days` days in the past.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `days` is negative.
/// Use [`days_from_now`] for future offsets.
#[inline]
pub fn days_ago(days: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(days, "days", "days_from_now")?;
    Ok(Local::now().date_naive() - Duration::days(days))
}

/// Returns the local date `days` days in the future.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `days` is negative.
/// Use [`days_ago`] for past offsets.
#[inline]
pub fn days_from_now(days: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(days, "days", "days_ago")?;
    Ok(Local::now().date_naive() + Duration::days(days))
}

/// Returns the local date `weeks` weeks in the past.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `weeks` is negative.
/// Use [`weeks_from_now`] for future offsets.
#[inline]
pub fn weeks_ago(weeks: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(weeks, "weeks", "weeks_from_now")?;
    Ok(Local::now().date_naive() - Duration::weeks(weeks))
}

/// Returns the local date `weeks` weeks in the future.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `weeks` is negative.
/// Use [`weeks_ago`] for past offsets.
#[inline]
pub fn weeks_from_now(weeks: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(weeks, "weeks", "weeks_ago")?;
    Ok(Local::now().date_naive() + Duration::weeks(weeks))
}

/// Returns yesterday's local date.
#[must_use]
#[inline]
pub fn yesterday() -> NaiveDate {
    Local::now().date_naive() - Duration::days(1)
}

/// Returns tomorrow's local date.
#[must_use]
#[inline]
pub fn tomorrow() -> NaiveDate {
    Local::now().date_naive() + Duration::days(1)
}

/// Returns the local date `months` calendar months in the past.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `months` is negative.
/// Returns [`TempusError::Overflow`] if `months` exceeds [`u32::MAX`].
/// Use [`months_from_now`] for future offsets.
#[inline]
pub fn months_ago(months: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(months, "months", "months_from_now")?;
    let months_u32 = u32::try_from(months).map_err(|_| TempusError::Overflow {
        unit: "months",
        value: months,
    })?;
    Ok(Local::now().date_naive() - Months::new(months_u32))
}

/// Returns the local date `months` calendar months in the future.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `months` is negative.
/// Returns [`TempusError::Overflow`] if `months` exceeds [`u32::MAX`].
/// Use [`months_ago`] for past offsets.
#[inline]
pub fn months_from_now(months: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(months, "months", "months_ago")?;
    let months_u32 = u32::try_from(months).map_err(|_| TempusError::Overflow {
        unit: "months",
        value: months,
    })?;
    Ok(Local::now().date_naive() + Months::new(months_u32))
}

/// Returns the local date `years` calendar years in the past.
///
/// Internally converts years to months. A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `years` is negative.
/// Returns [`TempusError::Overflow`] if the equivalent month count overflows.
/// Use [`years_from_now`] for future offsets.
#[inline]
pub fn years_ago(years: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(years, "years", "years_from_now")?;
    let months = u32::try_from(years.saturating_mul(12)).map_err(|_| TempusError::Overflow {
        unit: "years",
        value: years,
    })?;
    Ok(Local::now().date_naive() - Months::new(months))
}

/// Returns the local date `years` calendar years in the future.
///
/// Internally converts years to months. A value of `0` returns today.
///
/// # Errors
/// Returns [`TempusError::NegativeValue`] if `years` is negative.
/// Returns [`TempusError::Overflow`] if the equivalent month count overflows.
/// Use [`years_ago`] for past offsets.
#[inline]
pub fn years_from_now(years: i64) -> Result<NaiveDate, TempusError> {
    validate_positive(years, "years", "years_ago")?;
    let months = u32::try_from(years.saturating_mul(12)).map_err(|_| TempusError::Overflow {
        unit: "years",
        value: years,
    })?;
    Ok(Local::now().date_naive() + Months::new(months))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, Months};

    #[test]
    fn test_seconds_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::seconds(3);
        let result = seconds_ago(3).unwrap();
        let upper = Local::now() - Duration::seconds(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_seconds_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_ago(0).unwrap();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_seconds_ago_negative_returns_error() {
        assert_eq!(
            seconds_ago(-3).unwrap_err().to_string(),
            "seconds must be positive. Did you mean seconds_from_now(3)?"
        );
    }

    #[test]
    fn test_seconds_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::seconds(3);
        let result = seconds_from_now(3).unwrap();
        let upper = Local::now() + Duration::seconds(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_seconds_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_from_now(0).unwrap();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_seconds_from_now_negative_returns_error() {
        assert_eq!(
            seconds_from_now(-3).unwrap_err().to_string(),
            "seconds must be positive. Did you mean seconds_ago(3)?"
        );
    }

    #[test]
    fn test_minutes_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::minutes(3);
        let result = minutes_ago(3).unwrap();
        let upper = Local::now() - Duration::minutes(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_minutes_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_ago(0).unwrap();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_minutes_ago_negative_returns_error() {
        assert_eq!(
            minutes_ago(-3).unwrap_err().to_string(),
            "minutes must be positive. Did you mean minutes_from_now(3)?"
        );
    }

    #[test]
    fn test_minutes_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::minutes(3);
        let result = minutes_from_now(3).unwrap();
        let upper = Local::now() + Duration::minutes(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_minutes_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_from_now(0).unwrap();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_minutes_from_now_negative_returns_error() {
        assert_eq!(
            minutes_from_now(-3).unwrap_err().to_string(),
            "minutes must be positive. Did you mean minutes_ago(3)?"
        );
    }

    #[test]
    fn test_hours_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::hours(3);
        let result = hours_ago(3).unwrap();
        let upper = Local::now() - Duration::hours(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_hours_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = hours_ago(0).unwrap();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_hours_ago_negative_returns_error() {
        assert_eq!(
            hours_ago(-3).unwrap_err().to_string(),
            "hours must be positive. Did you mean hours_from_now(3)?"
        );
    }

    #[test]
    fn test_hours_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::hours(3);
        let result = hours_from_now(3).unwrap();
        let upper = Local::now() + Duration::hours(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_hours_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = hours_from_now(0).unwrap();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_hours_from_now_negative_returns_error() {
        assert_eq!(
            hours_from_now(-3).unwrap_err().to_string(),
            "hours must be positive. Did you mean hours_ago(3)?"
        );
    }

    #[test]
    fn test_days_ago_returns_correct_date() {
        let date = days_ago(3).unwrap();
        let expected = Local::now().date_naive() - Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_ago_with_zero_returns_today() {
        assert_eq!(days_ago(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_days_ago_negative_returns_error() {
        assert_eq!(
            days_ago(-3).unwrap_err().to_string(),
            "days must be positive. Did you mean days_from_now(3)?"
        );
    }

    #[test]
    fn test_days_from_now_returns_correct_date() {
        let date = days_from_now(3).unwrap();
        let expected = Local::now().date_naive() + Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_from_now_with_zero_returns_today() {
        assert_eq!(days_from_now(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_days_from_now_negative_returns_error() {
        assert_eq!(
            days_from_now(-3).unwrap_err().to_string(),
            "days must be positive. Did you mean days_ago(3)?"
        );
    }

    #[test]
    fn test_weeks_ago_returns_correct_date() {
        let date = weeks_ago(2).unwrap();
        let expected = Local::now().date_naive() - Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_ago_with_zero_returns_today() {
        assert_eq!(weeks_ago(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_ago_negative_returns_error() {
        assert_eq!(
            weeks_ago(-2).unwrap_err().to_string(),
            "weeks must be positive. Did you mean weeks_from_now(2)?"
        );
    }

    #[test]
    fn test_weeks_from_now_returns_correct_date() {
        let date = weeks_from_now(2).unwrap();
        let expected = Local::now().date_naive() + Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_from_now_with_zero_returns_today() {
        assert_eq!(weeks_from_now(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_from_now_negative_returns_error() {
        assert_eq!(
            weeks_from_now(-2).unwrap_err().to_string(),
            "weeks must be positive. Did you mean weeks_ago(2)?"
        );
    }

    #[test]
    fn test_yesterday_returns_previous_date() {
        let date = yesterday();
        let expected = Local::now().date_naive() - Duration::days(1);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_tomorrow_returns_next_date() {
        let date = tomorrow();
        let expected = Local::now().date_naive() + Duration::days(1);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_ago_returns_correct_date() {
        let date = months_ago(2).unwrap();
        let expected = Local::now().date_naive() - Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_ago_with_zero_returns_today() {
        assert_eq!(months_ago(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_months_ago_negative_returns_error() {
        assert_eq!(
            months_ago(-2).unwrap_err().to_string(),
            "months must be positive. Did you mean months_from_now(2)?"
        );
    }

    #[test]
    fn test_months_from_now_returns_correct_date() {
        let date = months_from_now(2).unwrap();
        let expected = Local::now().date_naive() + Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_from_now_with_zero_returns_today() {
        assert_eq!(months_from_now(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_months_from_now_negative_returns_error() {
        assert_eq!(
            months_from_now(-2).unwrap_err().to_string(),
            "months must be positive. Did you mean months_ago(2)?"
        );
    }

    #[test]
    fn test_months_ago_overflow_returns_error() {
        let result = months_ago(5_000_000_000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "months value 5000000000 is too large"
        );
    }

    #[test]
    fn test_months_from_now_overflow_returns_error() {
        let result = months_from_now(5_000_000_000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "months value 5000000000 is too large"
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
    fn test_years_from_now_overflow_returns_error() {
        let result = years_from_now(1_000_000_000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "years value 1000000000 is too large"
        );
    }

    #[test]
    fn test_years_ago_returns_correct_date() {
        let date = years_ago(2).unwrap();
        let expected = months_ago(24).unwrap();
        assert_eq!(date, expected);
    }

    #[test]
    fn test_years_ago_with_zero_returns_today() {
        assert_eq!(years_ago(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_years_ago_negative_returns_error() {
        assert_eq!(
            years_ago(-2).unwrap_err().to_string(),
            "years must be positive. Did you mean years_from_now(2)?"
        );
    }

    #[test]
    fn test_years_from_now_returns_correct_date() {
        let date = years_from_now(2).unwrap();
        let expected = months_from_now(24).unwrap();
        assert_eq!(date, expected);
    }

    #[test]
    fn test_years_from_now_with_zero_returns_today() {
        assert_eq!(years_from_now(0).unwrap(), Local::now().date_naive());
    }

    #[test]
    fn test_years_from_now_negative_returns_error() {
        assert_eq!(
            years_from_now(-2).unwrap_err().to_string(),
            "years must be positive. Did you mean years_ago(2)?"
        );
    }
}
