use crate::error::PeriodError;
use chrono::{DateTime, Duration, Local, Months, NaiveDate, NaiveTime};

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

/// A resolved point in time returned by every relative-date function.
///
/// Call `.as_datetime()`, `.as_date()`, or `.as_time()` to extract the
/// representation you need, or rely on the `From` / `Into` conversions.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), period::PeriodError> {
/// let r = period::days_ago(3)?;
/// let date     = r.as_date();     // NaiveDate       - just the calendar day
/// let datetime = r.as_datetime(); // DateTime<Local> - full timestamp
/// let time     = r.as_time();     // NaiveTime       - just the clock reading
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relative(DateTime<Local>);

impl Relative {
    /// The full local date-time.
    #[inline]
    #[must_use]
    pub fn as_datetime(self) -> DateTime<Local> {
        self.0
    }

    /// The calendar date, discarding the time-of-day component.
    #[inline]
    #[must_use]
    pub fn as_date(self) -> NaiveDate {
        self.0.date_naive()
    }

    /// The time-of-day, discarding the date component.
    #[inline]
    #[must_use]
    pub fn as_time(self) -> NaiveTime {
        self.0.time()
    }
}

impl From<Relative> for DateTime<Local> {
    fn from(r: Relative) -> Self {
        r.0
    }
}

impl From<Relative> for NaiveDate {
    fn from(r: Relative) -> Self {
        r.0.date_naive()
    }
}

impl From<Relative> for NaiveTime {
    fn from(r: Relative) -> Self {
        r.0.time()
    }
}

/// Returns a [`Relative`] moment `seconds` seconds in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `seconds` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`seconds_from_now`] for future offsets.
#[inline]
pub fn seconds_ago(seconds: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(seconds, "seconds", "seconds_from_now")?;
    let duration = Duration::try_seconds(seconds).ok_or(PeriodError::Overflow {
        unit: "seconds",
        value: seconds,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "seconds",
            value: seconds,
        })
}

/// Returns a [`Relative`] moment `seconds` seconds in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `seconds` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`seconds_ago`] for past offsets.
#[inline]
pub fn seconds_from_now(seconds: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(seconds, "seconds", "seconds_ago")?;
    let duration = Duration::try_seconds(seconds).ok_or(PeriodError::Overflow {
        unit: "seconds",
        value: seconds,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "seconds",
            value: seconds,
        })
}

/// Returns a [`Relative`] moment `minutes` minutes in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `minutes` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`minutes_from_now`] for future offsets.
#[inline]
pub fn minutes_ago(minutes: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(minutes, "minutes", "minutes_from_now")?;
    let duration = Duration::try_minutes(minutes).ok_or(PeriodError::Overflow {
        unit: "minutes",
        value: minutes,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "minutes",
            value: minutes,
        })
}

/// Returns a [`Relative`] moment `minutes` minutes in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `minutes` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`minutes_ago`] for past offsets.
#[inline]
pub fn minutes_from_now(minutes: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(minutes, "minutes", "minutes_ago")?;
    let duration = Duration::try_minutes(minutes).ok_or(PeriodError::Overflow {
        unit: "minutes",
        value: minutes,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "minutes",
            value: minutes,
        })
}

/// Returns a [`Relative`] moment `hours` hours in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `hours` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`hours_from_now`] for future offsets.
#[inline]
pub fn hours_ago(hours: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(hours, "hours", "hours_from_now")?;
    let duration = Duration::try_hours(hours).ok_or(PeriodError::Overflow {
        unit: "hours",
        value: hours,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "hours",
            value: hours,
        })
}

/// Returns a [`Relative`] moment `hours` hours in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `hours` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`hours_ago`] for past offsets.
#[inline]
pub fn hours_from_now(hours: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(hours, "hours", "hours_ago")?;
    let duration = Duration::try_hours(hours).ok_or(PeriodError::Overflow {
        unit: "hours",
        value: hours,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "hours",
            value: hours,
        })
}

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

/// Returns a [`Relative`] moment `weeks` weeks in the past.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `weeks` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`weeks_from_now`] for future offsets.
#[inline]
pub fn weeks_ago(weeks: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(weeks, "weeks", "weeks_from_now")?;
    let duration = Duration::try_weeks(weeks).ok_or(PeriodError::Overflow {
        unit: "weeks",
        value: weeks,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "weeks",
            value: weeks,
        })
}

/// Returns a [`Relative`] moment `weeks` weeks in the future.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `weeks` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`weeks_ago`] for past offsets.
#[inline]
pub fn weeks_from_now(weeks: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(weeks, "weeks", "weeks_ago")?;
    let duration = Duration::try_weeks(weeks).ok_or(PeriodError::Overflow {
        unit: "weeks",
        value: weeks,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
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

/// Returns a [`Relative`] moment `months` calendar months in the past.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `months` is negative.
/// Returns [`PeriodError::Overflow`] if `months` exceeds [`u32::MAX`] or the resulting date-time is out of range.
/// Use [`months_from_now`] for future offsets.
#[inline]
pub fn months_ago(months: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(months, "months", "months_from_now")?;
    let months_u32 = u32::try_from(months).map_err(|_| PeriodError::Overflow {
        unit: "months",
        value: months,
    })?;
    Local::now()
        .checked_sub_months(Months::new(months_u32))
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "months",
            value: months,
        })
}

/// Returns a [`Relative`] moment `months` calendar months in the future.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `months` is negative.
/// Returns [`PeriodError::Overflow`] if `months` exceeds [`u32::MAX`] or the resulting date-time is out of range.
/// Use [`months_ago`] for past offsets.
#[inline]
pub fn months_from_now(months: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(months, "months", "months_ago")?;
    let months_u32 = u32::try_from(months).map_err(|_| PeriodError::Overflow {
        unit: "months",
        value: months,
    })?;
    Local::now()
        .checked_add_months(Months::new(months_u32))
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "months",
            value: months,
        })
}

/// Returns a [`Relative`] moment `years` calendar years in the past.
///
/// Internally converts years to months. A value of `0` returns the current
/// date-time. Use `.as_date()` to get a [`NaiveDate`] if you do not need the
/// time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `years` is negative.
/// Returns [`PeriodError::Overflow`] if the equivalent month count overflows or the resulting date-time is out of range.
/// Use [`years_from_now`] for future offsets.
#[inline]
pub fn years_ago(years: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(years, "years", "years_from_now")?;
    let months = u32::try_from(years.saturating_mul(12)).map_err(|_| PeriodError::Overflow {
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
/// date-time. Use `.as_date()` to get a [`NaiveDate`] if you do not need the
/// time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `years` is negative.
/// Returns [`PeriodError::Overflow`] if the equivalent month count overflows or the resulting date-time is out of range.
/// Use [`years_ago`] for past offsets.
#[inline]
pub fn years_from_now(years: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(years, "years", "years_ago")?;
    let months = u32::try_from(years.saturating_mul(12)).map_err(|_| PeriodError::Overflow {
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
/// | >= 18 months   | `"N years ago"`   | `"in N years"`   |
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

    // -- seconds_ago ----------------------------------------------------------

    #[test]
    fn test_seconds_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::seconds(3);
        let result = seconds_ago(3).unwrap().as_datetime();
        let upper = Local::now() - Duration::seconds(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_seconds_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_ago(0).unwrap().as_datetime();
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

    // -- seconds_from_now -----------------------------------------------------

    #[test]
    fn test_seconds_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::seconds(3);
        let result = seconds_from_now(3).unwrap().as_datetime();
        let upper = Local::now() + Duration::seconds(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_seconds_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_from_now(0).unwrap().as_datetime();
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

    // -- minutes_ago ----------------------------------------------------------

    #[test]
    fn test_minutes_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::minutes(3);
        let result = minutes_ago(3).unwrap().as_datetime();
        let upper = Local::now() - Duration::minutes(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_minutes_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_ago(0).unwrap().as_datetime();
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

    // -- minutes_from_now -----------------------------------------------------

    #[test]
    fn test_minutes_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::minutes(3);
        let result = minutes_from_now(3).unwrap().as_datetime();
        let upper = Local::now() + Duration::minutes(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_minutes_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_from_now(0).unwrap().as_datetime();
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

    // -- hours_ago ------------------------------------------------------------

    #[test]
    fn test_hours_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::hours(3);
        let result = hours_ago(3).unwrap().as_datetime();
        let upper = Local::now() - Duration::hours(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_hours_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = hours_ago(0).unwrap().as_datetime();
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

    // -- hours_from_now -------------------------------------------------------

    #[test]
    fn test_hours_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::hours(3);
        let result = hours_from_now(3).unwrap().as_datetime();
        let upper = Local::now() + Duration::hours(3);

        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_hours_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = hours_from_now(0).unwrap().as_datetime();
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

    // -- days_ago -------------------------------------------------------------

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
        assert!(days_ago(200_000_000).is_err());
    }

    // -- days_from_now --------------------------------------------------------

    #[test]
    fn test_days_from_now_returns_correct_date() {
        let date = days_from_now(3).unwrap().as_date();
        let expected = Local::now().date_naive() + Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_from_now_with_zero_returns_today() {
        assert_eq!(days_from_now(0).unwrap().as_date(), Local::now().date_naive());
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
        assert!(days_from_now(200_000_000).is_err());
    }

    // -- weeks_ago ------------------------------------------------------------

    #[test]
    fn test_weeks_ago_returns_correct_date() {
        let date = weeks_ago(2).unwrap().as_date();
        let expected = Local::now().date_naive() - Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_ago_with_zero_returns_today() {
        assert_eq!(weeks_ago(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::weeks(2);
        let result = weeks_ago(2).unwrap().as_datetime();
        let upper = Local::now() - Duration::weeks(2);
        assert!(result >= lower);
        assert!(result <= upper);
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

    // -- weeks_from_now -------------------------------------------------------

    #[test]
    fn test_weeks_from_now_returns_correct_date() {
        let date = weeks_from_now(2).unwrap().as_date();
        let expected = Local::now().date_naive() + Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_from_now_with_zero_returns_today() {
        assert_eq!(weeks_from_now(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::weeks(2);
        let result = weeks_from_now(2).unwrap().as_datetime();
        let upper = Local::now() + Duration::weeks(2);
        assert!(result >= lower);
        assert!(result <= upper);
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

    // -- yesterday / tomorrow -------------------------------------------------

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

    // -- months_ago -----------------------------------------------------------

    #[test]
    fn test_months_ago_returns_correct_date() {
        let date = months_ago(2).unwrap().as_date();
        let expected = Local::now().date_naive() - Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_ago_with_zero_returns_today() {
        assert_eq!(months_ago(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_months_ago_returns_correct_datetime() {
        let lower = Local::now().checked_sub_months(Months::new(2)).unwrap();
        let result = months_ago(2).unwrap().as_datetime();
        let upper = Local::now().checked_sub_months(Months::new(2)).unwrap();
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_months_ago_negative_returns_error() {
        assert_eq!(
            months_ago(-2).unwrap_err().to_string(),
            "months must be positive. Did you mean months_from_now(2)?"
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

    // -- months_from_now ------------------------------------------------------

    #[test]
    fn test_months_from_now_returns_correct_date() {
        let date = months_from_now(2).unwrap().as_date();
        let expected = Local::now().date_naive() + Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_from_now_with_zero_returns_today() {
        assert_eq!(months_from_now(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_months_from_now_returns_correct_datetime() {
        let lower = Local::now().checked_add_months(Months::new(2)).unwrap();
        let result = months_from_now(2).unwrap().as_datetime();
        let upper = Local::now().checked_add_months(Months::new(2)).unwrap();
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_months_from_now_negative_returns_error() {
        assert_eq!(
            months_from_now(-2).unwrap_err().to_string(),
            "months must be positive. Did you mean months_ago(2)?"
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

    // -- years_ago ------------------------------------------------------------

    #[test]
    fn test_years_ago_returns_correct_date() {
        let date = years_ago(2).unwrap().as_date();
        let expected = months_ago(24).unwrap().as_date();
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

    // -- years_from_now -------------------------------------------------------

    #[test]
    fn test_years_from_now_returns_correct_date() {
        let date = years_from_now(2).unwrap().as_date();
        let expected = months_from_now(24).unwrap().as_date();
        assert_eq!(date, expected);
    }

    #[test]
    fn test_years_from_now_with_zero_returns_today() {
        assert_eq!(years_from_now(0).unwrap().as_date(), Local::now().date_naive());
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

    // -- Relative conversions -------------------------------------------------

    #[test]
    fn test_relative_as_date_matches_naive_date() {
        let r = days_ago(1).unwrap();
        let expected = Local::now().date_naive() - Duration::days(1);
        assert_eq!(r.as_date(), expected);
        assert_eq!(NaiveDate::from(r), expected);
    }

    #[test]
    fn test_relative_as_datetime_matches_datetime() {
        let lower = Local::now() - Duration::hours(1);
        let r = hours_ago(1).unwrap();
        let upper = Local::now() - Duration::hours(1);
        let dt: DateTime<Local> = r.into();
        assert!(dt >= lower);
        assert!(dt <= upper);
    }

    #[test]
    fn test_relative_as_time_is_consistent_with_as_datetime() {
        let r = hours_ago(2).unwrap();
        assert_eq!(r.as_time(), r.as_datetime().time());
    }

    // -- humanize -------------------------------------------------------------

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
        // 95 s -> n = 1 -> singular
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
        // 13 months past -- inside "< 18 months" bucket
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
        // 95 s -> n = 1 -> singular future form
        assert_eq!(humanize(future_dt(95)), "in 1 minute");
    }

    #[test]
    fn test_humanize_1_hour_singular_ago() {
        // 5401 s (approx 90 min) -> hours bucket, n = 1
        assert_eq!(humanize(past_dt(5401)), "1 hour ago");
    }

    #[test]
    fn test_humanize_in_1_hour_singular() {
        assert_eq!(humanize(future_dt(5401)), "in 1 hour");
    }

    #[test]
    fn test_humanize_1_day_singular_ago() {
        // 37 h -> days bucket, n = 1
        assert_eq!(humanize(past_dt(37 * 3_600)), "1 day ago");
    }

    #[test]
    fn test_humanize_in_1_day_singular() {
        assert_eq!(humanize(future_dt(37 * 3_600 + 30)), "in 1 day");
    }

    #[test]
    fn test_humanize_1_month_singular_ago() {
        // 46 days -> months bucket, n = 1
        assert_eq!(humanize(past_dt(46 * 86_400)), "1 month ago");
    }

    #[test]
    fn test_humanize_in_1_month_singular() {
        assert_eq!(humanize(future_dt(46 * 86_400 + 30)), "in 1 month");
    }

    #[test]
    fn test_humanize_1_year_singular_ago() {
        // 19 x 30 days -> years bucket, n = 1
        assert_eq!(humanize(past_dt(19 * 30 * 86_400)), "1 year ago");
    }

    #[test]
    fn test_humanize_in_1_year_singular() {
        assert_eq!(humanize(future_dt(19 * 30 * 86_400 + 30)), "in 1 year");
    }

    // -- humanize: exact threshold boundaries ---------------------------------

    #[test]
    fn test_humanize_exactly_30s_past_is_a_minute_ago() {
        // abs == 30 falls in the `abs < 90` bucket, not "just now"
        assert_eq!(humanize(past_dt(30)), "a minute ago");
    }

    #[test]
    fn test_humanize_exactly_30s_future_is_in_a_minute() {
        // +1 s buffer: time passes between future_dt() and humanize(), so use
        // 31 s to stay safely in the 30–89 s bucket rather than slipping to "just now".
        assert_eq!(humanize(future_dt(31)), "in a minute");
    }

    #[test]
    fn test_humanize_89s_past_is_a_minute_ago() {
        assert_eq!(humanize(past_dt(89)), "a minute ago");
    }

    #[test]
    fn test_humanize_44_minutes_past() {
        // 44 * 60 = 2640 s < 45 * 60 = 2700 → minutes bucket
        assert_eq!(humanize(past_dt(44 * 60)), "44 minutes ago");
    }

    #[test]
    fn test_humanize_exactly_45_minutes_past_is_an_hour_ago() {
        // abs == 2700 is NOT < 2700, falls into `abs < 90 * MINUTE`
        assert_eq!(humanize(past_dt(45 * 60)), "an hour ago");
    }

    #[test]
    fn test_humanize_exactly_45_minutes_future_is_in_an_hour() {
        // +30 s buffer: keeps the abs delta above 45 * 60 despite elapsed time.
        assert_eq!(humanize(future_dt(45 * 60 + 30)), "in an hour");
    }

    #[test]
    fn test_humanize_89_minutes_past_is_an_hour_ago() {
        // 89 * 60 = 5340 s < 90 * 60 = 5400 → still "an hour ago"
        assert_eq!(humanize(past_dt(89 * 60)), "an hour ago");
    }

    #[test]
    fn test_humanize_21_hours_past() {
        // 21 h < 22 h threshold → hours bucket
        assert_eq!(humanize(past_dt(21 * 3_600)), "21 hours ago");
    }

    #[test]
    fn test_humanize_exactly_22_hours_past_is_yesterday() {
        // abs == 22 * 3600 is NOT < 22 * HOUR → falls into `< 36h` → "yesterday"
        assert_eq!(humanize(past_dt(22 * 3_600)), "yesterday");
    }

    #[test]
    fn test_humanize_exactly_22_hours_future_is_tomorrow() {
        // +30 s buffer: keeps abs delta above 22 * HOUR despite elapsed time.
        assert_eq!(humanize(future_dt(22 * 3_600 + 30)), "tomorrow");
    }

    #[test]
    fn test_humanize_35_hours_past_is_yesterday() {
        assert_eq!(humanize(past_dt(35 * 3_600)), "yesterday");
    }

    #[test]
    fn test_humanize_exactly_36_hours_past_is_1_day_ago() {
        // abs == 36 * 3600 is NOT < 36h → days bucket, 129600 / 86400 = 1
        assert_eq!(humanize(past_dt(36 * 3_600)), "1 day ago");
    }

    #[test]
    fn test_humanize_24_days_past() {
        // 24 days < 25-day threshold → days bucket
        assert_eq!(humanize(past_dt(24 * 86_400)), "24 days ago");
    }

    #[test]
    fn test_humanize_exactly_25_days_past_is_a_month_ago() {
        // abs == 25 * 86400 is NOT < 25 * DAY → `< 45 * DAY` bucket
        assert_eq!(humanize(past_dt(25 * 86_400)), "a month ago");
    }

    #[test]
    fn test_humanize_exactly_25_days_future_is_in_a_month() {
        // +30 s buffer to survive elapsed time.
        assert_eq!(humanize(future_dt(25 * 86_400 + 30)), "in a month");
    }

    #[test]
    fn test_humanize_exactly_45_days_past_is_months_ago() {
        // abs == 45 * 86400 = 3888000, NOT < 45 * DAY → months bucket
        // 3888000 / (30 * 86400) = 3888000 / 2592000 = 1 → "1 month ago"
        assert_eq!(humanize(past_dt(45 * 86_400)), "1 month ago");
    }

    #[test]
    fn test_humanize_9_months_past() {
        // 9 * 30 * 86400 < 10 * 30 * 86400 → months bucket
        assert_eq!(humanize(past_dt(9 * 30 * 86_400)), "9 months ago");
    }

    #[test]
    fn test_humanize_exactly_10_months_past_is_a_year_ago() {
        // Falls into `< 18 * MONTH` bucket
        assert_eq!(humanize(past_dt(10 * 30 * 86_400)), "a year ago");
    }

    #[test]
    fn test_humanize_exactly_10_months_future_is_in_a_year() {
        // +30 s buffer to survive elapsed time.
        assert_eq!(humanize(future_dt(10 * 30 * 86_400 + 30)), "in a year");
    }

    #[test]
    fn test_humanize_exactly_18_months_past_is_years_ago() {
        // 18 * 30 * 86400 = 46656000; NOT < 18 * MONTH → years bucket
        // 46656000 / (365 * 86400) = 46656000 / 31536000 ≈ 1 → "1 year ago"
        assert_eq!(humanize(past_dt(18 * 30 * 86_400)), "1 year ago");
    }

    #[test]
    fn test_humanize_exactly_18_months_future_is_in_years() {
        assert_eq!(humanize(future_dt(18 * 30 * 86_400 + 30)), "in 1 year");
    }

    #[test]
    fn test_humanize_now_is_just_now() {
        // A datetime of exactly now (0 s delta) is "just now"
        assert_eq!(humanize(Local::now()), "just now");
    }

    // -- error type: field inspection -----------------------------------------

    #[test]
    fn test_negative_value_error_has_correct_value_field() {
        let err = days_ago(-5).unwrap_err();
        assert!(
            matches!(err, PeriodError::NegativeValue { value: 5, .. }),
            "expected value=5, got {err:?}"
        );
    }

    #[test]
    fn test_negative_value_error_has_correct_unit_field() {
        let err = hours_ago(-1).unwrap_err();
        assert!(
            matches!(err, PeriodError::NegativeValue { unit: "hours", .. }),
            "expected unit='hours', got {err:?}"
        );
    }

    #[test]
    fn test_negative_value_error_has_correct_suggestion_field() {
        let err = months_ago(-3).unwrap_err();
        assert!(
            matches!(
                err,
                PeriodError::NegativeValue {
                    suggestion: "months_from_now",
                    ..
                }
            ),
            "expected suggestion='months_from_now', got {err:?}"
        );
    }

    #[test]
    fn test_overflow_error_has_correct_unit_field() {
        let err = months_ago(5_000_000_000).unwrap_err();
        assert!(
            matches!(err, PeriodError::Overflow { unit: "months", .. }),
            "expected unit='months', got {err:?}"
        );
    }

    #[test]
    fn test_overflow_error_has_correct_value_field() {
        let err = months_ago(5_000_000_000).unwrap_err();
        assert!(
            matches!(
                err,
                PeriodError::Overflow {
                    value: 5_000_000_000,
                    ..
                }
            ),
            "expected value=5_000_000_000, got {err:?}"
        );
    }

    #[test]
    fn test_period_error_equality() {
        assert_eq!(days_ago(-1).unwrap_err(), days_ago(-1).unwrap_err());
        assert_eq!(
            months_ago(5_000_000_000).unwrap_err(),
            months_ago(5_000_000_000).unwrap_err()
        );
    }

    #[test]
    fn test_period_error_negative_value_ne_overflow() {
        assert_ne!(
            days_ago(-1).unwrap_err(),
            months_ago(5_000_000_000).unwrap_err()
        );
    }

    #[test]
    fn test_period_error_implements_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(days_ago(-1).unwrap_err());
        assert!(err.source().is_none());
    }

    // -- Relative: ordering and trait coverage --------------------------------

    #[test]
    fn test_relative_past_is_less_than_future() {
        let past = days_ago(1).unwrap();
        let future = days_from_now(1).unwrap();
        assert!(past < future);
    }

    #[test]
    fn test_relative_is_copy_can_use_twice() {
        let r = hours_ago(1).unwrap();
        let dt1 = r.as_datetime(); // uses Copy, r not moved
        let dt2 = r.as_datetime();
        assert_eq!(dt1, dt2);
    }

    #[test]
    fn test_relative_clone_equals_original() {
        let r = minutes_ago(5).unwrap();
        #[allow(clippy::clone_on_copy)]
        let cloned = r.clone();
        assert_eq!(r, cloned);
    }

    #[test]
    fn test_relative_debug_is_non_empty() {
        let r = seconds_ago(10).unwrap();
        assert!(!format!("{r:?}").is_empty());
    }

    #[test]
    fn test_relative_into_naive_time() {
        let r = hours_ago(1).unwrap();
        let t: NaiveTime = r.into();
        assert_eq!(t, r.as_time());
    }

    #[test]
    fn test_relative_into_naive_date() {
        let r = days_ago(3).unwrap();
        let d: NaiveDate = r.into();
        assert_eq!(d, r.as_date());
    }

    #[test]
    fn test_relative_as_time_matches_as_datetime_time() {
        let r = minutes_ago(90).unwrap();
        assert_eq!(r.as_time(), r.as_datetime().time());
    }

    #[test]
    fn test_relative_ordering_same_instant_is_equal() {
        // Two calls very close in time — use the same Relative value
        let r = seconds_ago(0).unwrap();
        assert_eq!(r, r);
    }

    // -- cross-unit equivalence -----------------------------------------------

    #[test]
    fn test_60_seconds_ago_same_date_as_1_minute_ago() {
        assert_eq!(
            seconds_ago(60).unwrap().as_date(),
            minutes_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_60_minutes_ago_same_date_as_1_hour_ago() {
        assert_eq!(
            minutes_ago(60).unwrap().as_date(),
            hours_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_24_hours_ago_same_date_as_1_day_ago() {
        assert_eq!(
            hours_ago(24).unwrap().as_date(),
            days_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_7_days_ago_same_date_as_1_week_ago() {
        assert_eq!(
            days_ago(7).unwrap().as_date(),
            weeks_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_12_months_ago_same_date_as_1_year_ago() {
        assert_eq!(
            months_ago(12).unwrap().as_date(),
            years_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_60_seconds_from_now_same_date_as_1_minute_from_now() {
        assert_eq!(
            seconds_from_now(60).unwrap().as_date(),
            minutes_from_now(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_7_days_from_now_same_date_as_1_week_from_now() {
        assert_eq!(
            days_from_now(7).unwrap().as_date(),
            weeks_from_now(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_12_months_from_now_same_date_as_1_year_from_now() {
        assert_eq!(
            months_from_now(12).unwrap().as_date(),
            years_from_now(1).unwrap().as_date()
        );
    }

    // -- arithmetic round-trips -----------------------------------------------

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
    fn test_weeks_ago_plus_n_equals_today() {
        let n = 3i64;
        let date = weeks_ago(n).unwrap().as_date();
        assert_eq!(date + Duration::weeks(n), Local::now().date_naive());
    }

    #[test]
    fn test_hours_ago_is_in_the_past() {
        let r = hours_ago(1).unwrap();
        assert!(r.as_datetime() < Local::now());
    }

    #[test]
    fn test_hours_from_now_is_in_the_future() {
        let r = hours_from_now(1).unwrap();
        assert!(r.as_datetime() > Local::now());
    }

    #[test]
    fn test_seconds_ago_is_in_the_past() {
        let r = seconds_ago(10).unwrap();
        assert!(r.as_datetime() < Local::now());
    }

    #[test]
    fn test_seconds_from_now_is_in_the_future() {
        let r = seconds_from_now(10).unwrap();
        assert!(r.as_datetime() > Local::now());
    }

    #[test]
    fn test_months_ago_is_in_the_past() {
        let r = months_ago(1).unwrap();
        assert!(r.as_date() < Local::now().date_naive());
    }

    #[test]
    fn test_months_from_now_is_in_the_future() {
        let r = months_from_now(1).unwrap();
        assert!(r.as_date() > Local::now().date_naive());
    }

    #[test]
    fn test_years_ago_is_in_the_past() {
        let r = years_ago(1).unwrap();
        assert!(r.as_date() < Local::now().date_naive());
    }

    #[test]
    fn test_years_from_now_is_in_the_future() {
        let r = years_from_now(1).unwrap();
        assert!(r.as_date() > Local::now().date_naive());
    }

    // -- large valid (non-overflow) values ------------------------------------

    #[test]
    fn test_seconds_ago_large_valid_value() {
        // 86 400 s = 1 day; should succeed and equal days_ago(1) date
        assert_eq!(
            seconds_ago(86_400).unwrap().as_date(),
            days_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_minutes_ago_large_valid_value() {
        // 1 440 min = 1 day
        assert_eq!(
            minutes_ago(1_440).unwrap().as_date(),
            days_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_hours_ago_large_valid_value() {
        // 240 h = 10 days
        assert_eq!(
            hours_ago(240).unwrap().as_date(),
            days_ago(10).unwrap().as_date()
        );
    }

    #[test]
    fn test_days_ago_large_valid_value() {
        // 365 days — well within range
        assert!(days_ago(365).is_ok());
    }

    #[test]
    fn test_weeks_from_now_large_valid_value() {
        // 52 weeks ≈ 1 year — should succeed
        assert!(weeks_from_now(52).is_ok());
    }

    // -- negative value edge cases: value=1 and large --------------------------

    #[test]
    fn test_seconds_ago_negative_one() {
        let err = seconds_ago(-1).unwrap_err();
        assert!(matches!(err, PeriodError::NegativeValue { value: 1, .. }));
    }

    #[test]
    fn test_minutes_from_now_negative_large() {
        let err = minutes_from_now(-1_000).unwrap_err();
        assert!(matches!(
            err,
            PeriodError::NegativeValue { value: 1_000, .. }
        ));
    }

    #[test]
    fn test_hours_ago_negative_one_has_hours_unit() {
        let err = hours_ago(-1).unwrap_err();
        assert!(matches!(err, PeriodError::NegativeValue { unit: "hours", .. }));
    }

    #[test]
    fn test_weeks_from_now_negative_one() {
        let err = weeks_from_now(-1).unwrap_err();
        assert!(matches!(err, PeriodError::NegativeValue { value: 1, .. }));
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

    // -- yesterday / tomorrow relationship ------------------------------------

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

    // -- humanize with Relative-derived datetime ------------------------------

    #[test]
    fn test_humanize_with_days_ago_relative() {
        let dt: DateTime<Local> = days_ago(3).unwrap().into();
        assert_eq!(humanize(dt), "3 days ago");
    }

    #[test]
    fn test_humanize_with_hours_from_now_relative() {
        // Verify that converting a Relative into a DateTime<Local> and passing
        // it to humanize produces a future-tense "in …" string.
        let r = hours_from_now(5).unwrap();
        let result = humanize(r.into());
        assert!(result.starts_with("in "), "expected 'in …', got: {result}");
    }
}
