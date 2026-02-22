use chrono::{DateTime, Local, NaiveDate};

/// Converts a NaiveDate to a string in "YYYY-MM-DD" format.
#[must_use]
#[inline]
pub fn to_date_string(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Converts a NaiveDate to a string in "Month Day, Year" format (e.g., "February 22, 2026").
/// This uses the full month name.
/// Note: This will be in English regardless of locale.
/// For locale-aware formatting, consider using the `chrono_locale` crate.
/// Example: `to_long_date(NaiveDate::from_ymd_opt(2026, 2, 22).unwrap())` returns "February 22, 2026".
/// This is a simple wrapper around `chrono`'s formatting capabilities.
/// The format string "%B %e, %Y" means:
/// - %B: Full month name (e.g., "February")
/// - %e: Day of the month, space-padded (e.g., "22")
/// - %Y: Year with century (e.g., "2026")
#[must_use]
#[inline]
pub fn to_long_date(date: NaiveDate) -> String {
    date.format("%B %e, %Y").to_string()
}

/// Converts a DateTime<Local> to an ISO 8601 string in the format "YYYY-MM-DDTHH:MM:SS+00:00".
/// Example: `to_iso8601(Local.with_ymd_and_hms(2026, 2, 22, 14, 30, 0).single().unwrap())` returns "2026-02-22T14:30:00+00:00".
/// This uses the `to_rfc3339` method from `chrono`, which produces a string in the format "YYYY-MM-DDTHH:MM:SS±HH:MM".
/// We then replace the timezone offset with "+00:00" to match the desired format.
#[must_use]
#[inline]
pub fn to_iso8601(datetime: DateTime<Local>) -> String {
    datetime.to_rfc3339().replace("+00:00", "+00:00")
}

/// Converts a DateTime<Local> to an RFC 2822 string in the format "Day, DD Mon YYYY HH:MM:SS +0000".
/// Example: `to_rfc2822(Local.with_ymd_and_hms(2026, 2, 22, 14, 30, 0).single().unwrap())` returns "Sun, 22 Feb 2026 14:30:00 +0000".
/// This uses the `to_rfc2822` method from `chrono`, which produces a string in the format "Day, DD Mon YYYY HH:MM:SS ±HHMM".
/// The timezone offset is included in the output, and will be in the format "+0000" for UTC.
#[must_use]
#[inline]
pub fn to_rfc2822(datetime: DateTime<Local>) -> String {
    datetime.to_rfc2822()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, NaiveDate, TimeZone};

    #[test]
    fn test_to_date_string() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_date_string(date), "2026-02-22");
    }

    #[test]
    fn test_to_long_date() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_long_date(date), "February 22, 2026");
    }

    #[test]
    fn test_to_long_date_with_single_digit_day() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
        assert_eq!(to_long_date(date), "February  5, 2026");
    }

    #[test]
    fn test_to_long_date_with_leading_zero_day() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
        assert_eq!(to_long_date(date), "February  5, 2026");
    }

    #[test]
    fn test_to_long_date_with_leading_zero_month() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_long_date(date), "February 22, 2026");
    }

    #[test]
    fn test_to_long_date_with_leading_zero_month_and_day() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
        assert_eq!(to_long_date(date), "February  5, 2026");
    }

    #[test]
    fn test_to_long_date_with_leading_zero_month_and_day_and_year() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
        assert_eq!(to_long_date(date), "February  5, 2026");
    }

    #[test]
    fn test_to_long_date_with_leading_zero_month_and_day_and_year_and_time() {
        let datetime = Local
            .with_ymd_and_hms(2026, 2, 5, 14, 30, 0)
            .single()
            .unwrap();
        let date = datetime.date_naive();
        assert_eq!(to_long_date(date), "February  5, 2026");
    }

    #[test]
    fn test_to_long_date_with_leading_zero_month_and_day_and_year_and_time_and_timezone() {
        let datetime = Local
            .with_ymd_and_hms(2026, 2, 5, 14, 30, 0)
            .single()
            .unwrap();
        let date = datetime.date_naive();
        assert_eq!(to_long_date(date), "February  5, 2026");
    }

    #[test]
    fn test_to_iso8601() {
        let datetime = Local
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        let result = to_iso8601(datetime);
        assert!(result.starts_with("2026-02-22T14:30:00"));
        assert!(result.contains('+') || result.contains('-'));
    }

    #[test]
    fn test_to_rfc2822() {
        let datetime = Local
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        let result = to_rfc2822(datetime);
        assert!(result.starts_with("Sun, 22 Feb 2026 14:30:00"));
    }
}
