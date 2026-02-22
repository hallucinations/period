use crate::error::PeriodError;
use chrono::{DateTime, Duration, Local, Months, NaiveDate};

fn validate_non_negative(
    value: i64,
    unit: &'static str,
    suggestion: &'static str,
) -> Result<(), PeriodError> {
    if value < 0 {
        return Err(PeriodError::NegativeValue {
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
/// Returns [`PeriodError::NegativeValue`] if `seconds` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`seconds_from_now`] for future offsets.
#[inline]
pub fn seconds_ago(seconds: i64) -> Result<DateTime<Local>, PeriodError> {
    validate_non_negative(seconds, "seconds", "seconds_from_now")?;
    let duration = Duration::try_seconds(seconds).ok_or(PeriodError::Overflow {
        unit: "seconds",
        value: seconds,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "seconds",
            value: seconds,
        })
}

/// Returns the local date-time `seconds` seconds in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `seconds` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`seconds_ago`] for past offsets.
#[inline]
pub fn seconds_from_now(seconds: i64) -> Result<DateTime<Local>, PeriodError> {
    validate_non_negative(seconds, "seconds", "seconds_ago")?;
    let duration = Duration::try_seconds(seconds).ok_or(PeriodError::Overflow {
        unit: "seconds",
        value: seconds,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "seconds",
            value: seconds,
        })
}

/// Returns the local date-time `minutes` minutes in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `minutes` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`minutes_from_now`] for future offsets.
#[inline]
pub fn minutes_ago(minutes: i64) -> Result<DateTime<Local>, PeriodError> {
    validate_non_negative(minutes, "minutes", "minutes_from_now")?;
    let duration = Duration::try_minutes(minutes).ok_or(PeriodError::Overflow {
        unit: "minutes",
        value: minutes,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "minutes",
            value: minutes,
        })
}

/// Returns the local date-time `minutes` minutes in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `minutes` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`minutes_ago`] for past offsets.
#[inline]
pub fn minutes_from_now(minutes: i64) -> Result<DateTime<Local>, PeriodError> {
    validate_non_negative(minutes, "minutes", "minutes_ago")?;
    let duration = Duration::try_minutes(minutes).ok_or(PeriodError::Overflow {
        unit: "minutes",
        value: minutes,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "minutes",
            value: minutes,
        })
}

/// Returns the local date-time `hours` hours in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `hours` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`hours_from_now`] for future offsets.
#[inline]
pub fn hours_ago(hours: i64) -> Result<DateTime<Local>, PeriodError> {
    validate_non_negative(hours, "hours", "hours_from_now")?;
    let duration = Duration::try_hours(hours).ok_or(PeriodError::Overflow {
        unit: "hours",
        value: hours,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "hours",
            value: hours,
        })
}

/// Returns the local date-time `hours` hours in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `hours` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`hours_ago`] for past offsets.
#[inline]
pub fn hours_from_now(hours: i64) -> Result<DateTime<Local>, PeriodError> {
    validate_non_negative(hours, "hours", "hours_ago")?;
    let duration = Duration::try_hours(hours).ok_or(PeriodError::Overflow {
        unit: "hours",
        value: hours,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "hours",
            value: hours,
        })
}

/// Returns the local date `days` days in the past.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `days` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date is out of range.
/// Use [`days_from_now`] for future offsets.
#[inline]
pub fn days_ago(days: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(days, "days", "days_from_now")?;
    let duration = Duration::try_days(days).ok_or(PeriodError::Overflow {
        unit: "days",
        value: days,
    })?;
    Local::now()
        .date_naive()
        .checked_sub_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "days",
            value: days,
        })
}

/// Returns the local date `days` days in the future.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `days` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date is out of range.
/// Use [`days_ago`] for past offsets.
#[inline]
pub fn days_from_now(days: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(days, "days", "days_ago")?;
    let duration = Duration::try_days(days).ok_or(PeriodError::Overflow {
        unit: "days",
        value: days,
    })?;
    Local::now()
        .date_naive()
        .checked_add_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "days",
            value: days,
        })
}

/// Returns the local date `weeks` weeks in the past.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `weeks` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date is out of range.
/// Use [`weeks_from_now`] for future offsets.
#[inline]
pub fn weeks_ago(weeks: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(weeks, "weeks", "weeks_from_now")?;
    let duration = Duration::try_weeks(weeks).ok_or(PeriodError::Overflow {
        unit: "weeks",
        value: weeks,
    })?;
    Local::now()
        .date_naive()
        .checked_sub_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "weeks",
            value: weeks,
        })
}

/// Returns the local date `weeks` weeks in the future.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `weeks` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date is out of range.
/// Use [`weeks_ago`] for past offsets.
#[inline]
pub fn weeks_from_now(weeks: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(weeks, "weeks", "weeks_ago")?;
    let duration = Duration::try_weeks(weeks).ok_or(PeriodError::Overflow {
        unit: "weeks",
        value: weeks,
    })?;
    Local::now()
        .date_naive()
        .checked_add_signed(duration)
        .ok_or(PeriodError::Overflow {
            unit: "weeks",
            value: weeks,
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

/// Returns the local date `months` calendar months in the past.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `months` is negative.
/// Returns [`PeriodError::Overflow`] if `months` exceeds [`u32::MAX`] or the resulting date is out of range.
/// Use [`months_from_now`] for future offsets.
#[inline]
pub fn months_ago(months: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(months, "months", "months_from_now")?;
    let months_u32 = u32::try_from(months).map_err(|_| PeriodError::Overflow {
        unit: "months",
        value: months,
    })?;
    Local::now()
        .date_naive()
        .checked_sub_months(Months::new(months_u32))
        .ok_or(PeriodError::Overflow {
            unit: "months",
            value: months,
        })
}

/// Returns the local date `months` calendar months in the future.
///
/// A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `months` is negative.
/// Returns [`PeriodError::Overflow`] if `months` exceeds [`u32::MAX`] or the resulting date is out of range.
/// Use [`months_ago`] for past offsets.
#[inline]
pub fn months_from_now(months: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(months, "months", "months_ago")?;
    let months_u32 = u32::try_from(months).map_err(|_| PeriodError::Overflow {
        unit: "months",
        value: months,
    })?;
    Local::now()
        .date_naive()
        .checked_add_months(Months::new(months_u32))
        .ok_or(PeriodError::Overflow {
            unit: "months",
            value: months,
        })
}

/// Returns the local date `years` calendar years in the past.
///
/// Internally converts years to months. A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `years` is negative.
/// Returns [`PeriodError::Overflow`] if the equivalent month count overflows or the resulting date is out of range.
/// Use [`years_from_now`] for future offsets.
#[inline]
pub fn years_ago(years: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(years, "years", "years_from_now")?;
    let months = u32::try_from(years.saturating_mul(12)).map_err(|_| PeriodError::Overflow {
        unit: "years",
        value: years,
    })?;
    Local::now()
        .date_naive()
        .checked_sub_months(Months::new(months))
        .ok_or(PeriodError::Overflow {
            unit: "years",
            value: years,
        })
}

/// Returns the local date `years` calendar years in the future.
///
/// Internally converts years to months. A value of `0` returns today.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `years` is negative.
/// Returns [`PeriodError::Overflow`] if the equivalent month count overflows or the resulting date is out of range.
/// Use [`years_ago`] for past offsets.
#[inline]
pub fn years_from_now(years: i64) -> Result<NaiveDate, PeriodError> {
    validate_non_negative(years, "years", "years_ago")?;
    let months = u32::try_from(years.saturating_mul(12)).map_err(|_| PeriodError::Overflow {
        unit: "years",
        value: years,
    })?;
    Local::now()
        .date_naive()
        .checked_add_months(Months::new(months))
        .ok_or(PeriodError::Overflow {
            unit: "years",
            value: years,
        })
}

/// Returns a human-readable relative-time string for `datetime`.
///
/// Past datetimes produce strings like `"3 minutes ago"` or `"yesterday"`.
/// Future datetimes produce strings like `"in 3 minutes"` or `"tomorrow"`.
/// A datetime within 30 seconds of now returns `"just now"` regardless of direction.
///
/// The bucketing thresholds:
///
/// | Absolute delta | Past              | Future           |
/// |----------------|-------------------|------------------|
/// | < 30 s         | `"just now"`      | `"just now"`     |
/// | < 90 s         | `"a minute ago"`  | `"in a minute"`  |
/// | < 45 min       | `"N minutes ago"` | `"in N minutes"` |
/// | < 90 min       | `"an hour ago"`   | `"in an hour"`   |
/// | < 22 h         | `"N hours ago"`   | `"in N hours"`   |
/// | < 36 h         | `"yesterday"`     | `"tomorrow"`     |
/// | < 25 days      | `"N days ago"`    | `"in N days"`    |
/// | < 45 days      | `"a month ago"`   | `"in a month"`   |
/// | < 10 months    | `"N months ago"`  | `"in N months"`  |
/// | < 18 months    | `"a year ago"`    | `"in a year"`    |
/// | ≥ 18 months    | `"N years ago"`   | `"in N years"`   |
#[inline]
#[must_use]
pub fn humanize(datetime: DateTime<Local>) -> String {
    const MINUTE: i64 = 60;
    const HOUR: i64 = 3_600;
    const DAY: i64 = 86_400;
    const MONTH: i64 = 30 * DAY;
    const YEAR: i64 = 365 * DAY;

    let secs = Local::now().signed_duration_since(datetime).num_seconds();
    let is_past = secs >= 0;
    let abs = secs.saturating_abs();

    if abs < 30 {
        "just now".to_string()
    } else if abs < 90 {
        if is_past {
            "a minute ago".to_string()
        } else {
            "in a minute".to_string()
        }
    } else if abs < 45 * MINUTE {
        let n = abs / MINUTE;
        let unit = if n == 1 { "minute" } else { "minutes" };
        if is_past {
            format!("{n} {unit} ago")
        } else {
            format!("in {n} {unit}")
        }
    } else if abs < 90 * MINUTE {
        if is_past {
            "an hour ago".to_string()
        } else {
            "in an hour".to_string()
        }
    } else if abs < 22 * HOUR {
        let n = abs / HOUR;
        let unit = if n == 1 { "hour" } else { "hours" };
        if is_past {
            format!("{n} {unit} ago")
        } else {
            format!("in {n} {unit}")
        }
    } else if abs < 36 * HOUR {
        if is_past {
            "yesterday".to_string()
        } else {
            "tomorrow".to_string()
        }
    } else if abs < 25 * DAY {
        let n = abs / DAY;
        let unit = if n == 1 { "day" } else { "days" };
        if is_past {
            format!("{n} {unit} ago")
        } else {
            format!("in {n} {unit}")
        }
    } else if abs < 45 * DAY {
        if is_past {
            "a month ago".to_string()
        } else {
            "in a month".to_string()
        }
    } else if abs < 10 * MONTH {
        let n = abs / MONTH;
        let unit = if n == 1 { "month" } else { "months" };
        if is_past {
            format!("{n} {unit} ago")
        } else {
            format!("in {n} {unit}")
        }
    } else if abs < 18 * MONTH {
        if is_past {
            "a year ago".to_string()
        } else {
            "in a year".to_string()
        }
    } else {
        let n = abs / YEAR;
        let unit = if n == 1 { "year" } else { "years" };
        if is_past {
            format!("{n} {unit} ago")
        } else {
            format!("in {n} {unit}")
        }
    }
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
    fn test_seconds_ago_overflow_returns_error() {
        assert!(seconds_ago(i64::MAX).is_err());
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
    fn test_seconds_from_now_overflow_returns_error() {
        assert!(seconds_from_now(i64::MAX).is_err());
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
    fn test_minutes_ago_overflow_returns_error() {
        assert!(minutes_ago(i64::MAX).is_err());
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
    fn test_minutes_from_now_overflow_returns_error() {
        assert!(minutes_from_now(i64::MAX).is_err());
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
    fn test_hours_ago_overflow_returns_error() {
        assert!(hours_ago(i64::MAX).is_err());
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
    fn test_hours_from_now_overflow_returns_error() {
        assert!(hours_from_now(i64::MAX).is_err());
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
    fn test_days_ago_overflow_returns_error() {
        assert!(days_ago(200_000_000).is_err());
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
    fn test_days_from_now_overflow_returns_error() {
        assert!(days_from_now(200_000_000).is_err());
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
    fn test_weeks_ago_overflow_returns_error() {
        assert!(weeks_ago(30_000_000).is_err());
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
    fn test_weeks_from_now_overflow_returns_error() {
        assert!(weeks_from_now(30_000_000).is_err());
    }

    #[test]
    fn test_yesterday_returns_previous_date() {
        let date = yesterday();
        let expected = Local::now().date_naive().pred_opt().unwrap();
        assert_eq!(date, expected);
    }

    #[test]
    fn test_tomorrow_returns_next_date() {
        let date = tomorrow();
        let expected = Local::now().date_naive().succ_opt().unwrap();
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

    // ── humanize ────────────────────────────────────────────────────────────

    fn past_dt(secs: i64) -> DateTime<Local> {
        Local::now() - Duration::seconds(secs)
    }

    fn future_dt(secs: i64) -> DateTime<Local> {
        Local::now() + Duration::seconds(secs)
    }

    #[test]
    fn test_humanize_just_now_past() {
        assert_eq!(humanize(past_dt(10)), "just now");
    }

    #[test]
    fn test_humanize_just_now_future() {
        assert_eq!(humanize(future_dt(10)), "just now");
    }

    #[test]
    fn test_humanize_a_minute_ago() {
        assert_eq!(humanize(past_dt(60)), "a minute ago");
    }

    #[test]
    fn test_humanize_in_a_minute() {
        assert_eq!(humanize(future_dt(60)), "in a minute");
    }

    #[test]
    fn test_humanize_minutes_ago() {
        assert_eq!(humanize(past_dt(5 * 60)), "5 minutes ago");
    }

    #[test]
    fn test_humanize_in_minutes() {
        // +30 s buffer: num_seconds() truncates toward zero, so a tiny
        // sub-second gap between future_dt() and humanize() would otherwise
        // shave off one second and drop the floor from 5 to 4.
        assert_eq!(humanize(future_dt(5 * 60 + 30)), "in 5 minutes");
    }

    #[test]
    fn test_humanize_1_minute_singular_ago() {
        // 95 s → n = 1 → singular
        assert_eq!(humanize(past_dt(95)), "1 minute ago");
    }

    #[test]
    fn test_humanize_an_hour_ago() {
        assert_eq!(humanize(past_dt(60 * 60)), "an hour ago");
    }

    #[test]
    fn test_humanize_in_an_hour() {
        assert_eq!(humanize(future_dt(60 * 60)), "in an hour");
    }

    #[test]
    fn test_humanize_hours_ago() {
        assert_eq!(humanize(past_dt(5 * 3600)), "5 hours ago");
    }

    #[test]
    fn test_humanize_in_hours() {
        assert_eq!(humanize(future_dt(5 * 3600 + 30)), "in 5 hours");
    }

    #[test]
    fn test_humanize_yesterday() {
        assert_eq!(humanize(past_dt(24 * 3600)), "yesterday");
    }

    #[test]
    fn test_humanize_tomorrow() {
        assert_eq!(humanize(future_dt(24 * 3600)), "tomorrow");
    }

    #[test]
    fn test_humanize_days_ago() {
        assert_eq!(humanize(past_dt(5 * 86_400)), "5 days ago");
    }

    #[test]
    fn test_humanize_in_days() {
        assert_eq!(humanize(future_dt(5 * 86_400 + 30)), "in 5 days");
    }

    #[test]
    fn test_humanize_a_month_ago() {
        assert_eq!(humanize(past_dt(30 * 86_400)), "a month ago");
    }

    #[test]
    fn test_humanize_in_a_month() {
        assert_eq!(humanize(future_dt(30 * 86_400)), "in a month");
    }

    #[test]
    fn test_humanize_months_ago() {
        assert_eq!(humanize(past_dt(3 * 30 * 86_400)), "3 months ago");
    }

    #[test]
    fn test_humanize_in_months() {
        assert_eq!(humanize(future_dt(3 * 30 * 86_400 + 30)), "in 3 months");
    }

    #[test]
    fn test_humanize_a_year_ago() {
        // 13 months past — inside "< 18 months" bucket
        assert_eq!(humanize(past_dt(13 * 30 * 86_400)), "a year ago");
    }

    #[test]
    fn test_humanize_in_a_year() {
        assert_eq!(humanize(future_dt(13 * 30 * 86_400)), "in a year");
    }

    #[test]
    fn test_humanize_years_ago() {
        assert_eq!(humanize(past_dt(3 * 365 * 86_400)), "3 years ago");
    }

    #[test]
    fn test_humanize_in_years() {
        assert_eq!(humanize(future_dt(3 * 365 * 86_400 + 30)), "in 3 years");
    }

    #[test]
    fn test_humanize_in_1_minute_singular() {
        // 95 s → n = 1 → singular future form
        assert_eq!(humanize(future_dt(95)), "in 1 minute");
    }

    #[test]
    fn test_humanize_1_hour_singular_ago() {
        // 5401 s (≈90 min) → hours bucket, n = 1
        assert_eq!(humanize(past_dt(5401)), "1 hour ago");
    }

    #[test]
    fn test_humanize_in_1_hour_singular() {
        assert_eq!(humanize(future_dt(5401)), "in 1 hour");
    }

    #[test]
    fn test_humanize_1_day_singular_ago() {
        // 37 h → days bucket, n = 1
        assert_eq!(humanize(past_dt(37 * 3_600)), "1 day ago");
    }

    #[test]
    fn test_humanize_in_1_day_singular() {
        assert_eq!(humanize(future_dt(37 * 3_600 + 30)), "in 1 day");
    }

    #[test]
    fn test_humanize_1_month_singular_ago() {
        // 46 days → months bucket, n = 1
        assert_eq!(humanize(past_dt(46 * 86_400)), "1 month ago");
    }

    #[test]
    fn test_humanize_in_1_month_singular() {
        assert_eq!(humanize(future_dt(46 * 86_400 + 30)), "in 1 month");
    }

    #[test]
    fn test_humanize_1_year_singular_ago() {
        // 19 × 30 days → years bucket, n = 1
        assert_eq!(humanize(past_dt(19 * 30 * 86_400)), "1 year ago");
    }

    #[test]
    fn test_humanize_in_1_year_singular() {
        assert_eq!(humanize(future_dt(19 * 30 * 86_400 + 30)), "in 1 year");
    }
}
