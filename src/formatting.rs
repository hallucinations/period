use chrono::{DateTime, Local, NaiveDate};

/// Converts a [`NaiveDate`] to an ISO 8601 date string (`YYYY-MM-DD`).
#[must_use]
#[inline]
pub fn to_date_string(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Converts a [`NaiveDate`] to a long-form date string (e.g. `"February 22, 2026"`).
///
/// Uses `%e` for the day, which space-pads single-digit days (`"February  5, 2026"`).
/// Output is always in English regardless of system locale.
#[must_use]
#[inline]
pub fn to_long_date(date: NaiveDate) -> String {
    date.format("%B %e, %Y").to_string()
}

/// Converts a [`DateTime<Local>`] to an RFC 3339 / ISO 8601 string
/// (e.g. `"2026-02-22T14:30:00+05:30"`).
#[must_use]
#[inline]
pub fn to_iso8601(datetime: DateTime<Local>) -> String {
    datetime.to_rfc3339()
}

/// Converts a [`DateTime<Local>`] to an RFC 2822 string
/// (e.g. `"Sun, 22 Feb 2026 14:30:00 -0600"`).
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
    fn test_to_long_date_single_digit_day() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
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
