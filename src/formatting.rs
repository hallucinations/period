use chrono::{DateTime, Datelike, NaiveDate, TimeZone};

/// Converts a [`NaiveDate`] to an ISO 8601 date string (`YYYY-MM-DD`).
#[must_use]
#[inline]
pub fn to_date_string(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Converts a [`NaiveDate`] to a long-form date string (e.g. `"February 22, 2026"`).
///
/// Single-digit days are not padded: `"February 5, 2026"`, not `"February  5, 2026"`.
/// Output is always in English regardless of system locale.
#[must_use]
#[inline]
pub fn to_long_date(date: NaiveDate) -> String {
    format!(
        "{} {}, {}",
        date.format("%B"),
        date.day(),
        date.format("%Y")
    )
}

/// Converts a [`NaiveDate`] to a short-form date string (e.g. `"Feb 22, 2026"`).
///
/// Single-digit days are not padded: `"Feb 5, 2026"`, not `"Feb  5, 2026"`.
/// Output is always in English regardless of system locale.
#[must_use]
#[inline]
pub fn to_short_date(date: NaiveDate) -> String {
    format!(
        "{} {}, {}",
        date.format("%b"),
        date.day(),
        date.format("%Y")
    )
}

/// Converts a [`DateTime`] to an RFC 3339 / ISO 8601 string
/// (e.g. `"2026-02-22T14:30:00+05:30"`).
///
/// Accepts any timezone — [`chrono::Local`], [`chrono::Utc`], [`chrono::FixedOffset`], etc.
#[must_use]
#[inline]
pub fn to_iso8601<Tz: TimeZone>(datetime: &DateTime<Tz>) -> String {
    datetime.to_rfc3339()
}

/// Converts a [`DateTime`] to an RFC 2822 string
/// (e.g. `"Sun, 22 Feb 2026 14:30:00 -0600"`).
///
/// Accepts any timezone — [`chrono::Local`], [`chrono::Utc`], [`chrono::FixedOffset`], etc.
#[must_use]
#[inline]
pub fn to_rfc2822<Tz: TimeZone>(datetime: &DateTime<Tz>) -> String {
    datetime.to_rfc2822()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, Local, NaiveDate, TimeZone};

    // -- to_date_string -------------------------------------------------------

    #[test]
    fn test_to_date_string() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_date_string(date), "2026-02-22");
    }

    #[test]
    fn test_to_date_string_single_digit_month_and_day() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap();
        assert_eq!(to_date_string(date), "2026-01-05");
    }

    #[test]
    fn test_to_date_string_year_end() {
        let date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        assert_eq!(to_date_string(date), "2026-12-31");
    }

    #[test]
    fn test_to_date_string_leap_day() {
        let date = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
        assert_eq!(to_date_string(date), "2028-02-29");
    }

    // -- to_long_date ---------------------------------------------------------

    #[test]
    fn test_to_long_date() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_long_date(date), "February 22, 2026");
    }

    #[test]
    fn test_to_long_date_single_digit_day() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
        assert_eq!(to_long_date(date), "February 5, 2026");
    }

    #[test]
    fn test_to_long_date_january() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        assert_eq!(to_long_date(date), "January 1, 2026");
    }

    #[test]
    fn test_to_long_date_december() {
        let date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        assert_eq!(to_long_date(date), "December 31, 2026");
    }

    #[test]
    fn test_to_long_date_leap_day() {
        let date = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
        assert_eq!(to_long_date(date), "February 29, 2028");
    }

    // -- to_short_date --------------------------------------------------------

    #[test]
    fn test_to_short_date() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_short_date(date), "Feb 22, 2026");
    }

    #[test]
    fn test_to_short_date_single_digit_day() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
        assert_eq!(to_short_date(date), "Feb 5, 2026");
    }

    #[test]
    fn test_to_short_date_january() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        assert_eq!(to_short_date(date), "Jan 1, 2026");
    }

    #[test]
    fn test_to_short_date_year_end() {
        let date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        assert_eq!(to_short_date(date), "Dec 31, 2026");
    }

    #[test]
    fn test_to_short_date_leap_day() {
        let date = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
        assert_eq!(to_short_date(date), "Feb 29, 2028");
    }

    // -- to_iso8601 -----------------------------------------------------------

    #[test]
    fn test_to_iso8601_local() {
        let datetime = Local
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        let result = to_iso8601(&datetime);
        assert!(
            result.starts_with("2026-02-22T14:30:00"),
            "unexpected iso8601: {result}"
        );
    }

    #[test]
    fn test_to_iso8601_utc() {
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        assert_eq!(to_iso8601(&datetime), "2026-02-22T14:30:00+00:00");
    }

    #[test]
    fn test_to_iso8601_positive_offset() {
        let tz = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(); // UTC+5:30 (India)
        let datetime = tz
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        assert_eq!(to_iso8601(&datetime), "2026-02-22T14:30:00+05:30");
    }

    #[test]
    fn test_to_iso8601_negative_offset() {
        let tz = FixedOffset::west_opt(6 * 3600).unwrap(); // UTC-6 (CST)
        let datetime = tz
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        assert_eq!(to_iso8601(&datetime), "2026-02-22T14:30:00-06:00");
    }

    #[test]
    fn test_to_iso8601_midnight() {
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).single().unwrap();
        assert_eq!(to_iso8601(&datetime), "2026-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_to_iso8601_leap_day() {
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz.with_ymd_and_hms(2028, 2, 29, 12, 0, 0).single().unwrap();
        assert_eq!(to_iso8601(&datetime), "2028-02-29T12:00:00+00:00");
    }

    #[test]
    fn test_to_iso8601_same_instant_different_offset() {
        // UTC midnight and IST 05:30 represent the same instant
        let utc = FixedOffset::east_opt(0).unwrap();
        let ist = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
        let dt_utc = utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).single().unwrap();
        let dt_ist = ist.with_ymd_and_hms(2026, 1, 1, 5, 30, 0).single().unwrap();
        // Wall-clock strings differ ...
        assert_eq!(to_iso8601(&dt_utc), "2026-01-01T00:00:00+00:00");
        assert_eq!(to_iso8601(&dt_ist), "2026-01-01T05:30:00+05:30");
        // ... but they represent the same UTC instant
        assert_eq!(dt_utc.to_utc(), dt_ist.to_utc());
    }

    // -- to_rfc2822 -----------------------------------------------------------

    #[test]
    fn test_to_rfc2822_local() {
        let datetime = Local
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        let result = to_rfc2822(&datetime);
        assert!(
            result.starts_with("Sun, 22 Feb 2026 14:30:00"),
            "unexpected rfc2822: {result}"
        );
    }

    #[test]
    fn test_to_rfc2822_utc() {
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        assert_eq!(to_rfc2822(&datetime), "Sun, 22 Feb 2026 14:30:00 +0000");
    }

    #[test]
    fn test_to_rfc2822_positive_offset() {
        let tz = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(); // UTC+5:30 (India)
        let datetime = tz
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        assert_eq!(to_rfc2822(&datetime), "Sun, 22 Feb 2026 14:30:00 +0530");
    }

    #[test]
    fn test_to_rfc2822_negative_offset() {
        let tz = FixedOffset::west_opt(6 * 3600).unwrap(); // UTC-6 (CST)
        let datetime = tz
            .with_ymd_and_hms(2026, 2, 22, 14, 30, 0)
            .single()
            .unwrap();
        assert_eq!(to_rfc2822(&datetime), "Sun, 22 Feb 2026 14:30:00 -0600");
    }

    #[test]
    fn test_to_rfc2822_weekday_monday() {
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz.with_ymd_and_hms(2026, 2, 23, 9, 0, 0).single().unwrap(); // Monday
        assert_eq!(to_rfc2822(&datetime), "Mon, 23 Feb 2026 09:00:00 +0000");
    }

    #[test]
    fn test_to_rfc2822_weekday_friday() {
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz.with_ymd_and_hms(2026, 2, 27, 9, 0, 0).single().unwrap(); // Friday
        assert_eq!(to_rfc2822(&datetime), "Fri, 27 Feb 2026 09:00:00 +0000");
    }

    #[test]
    fn test_to_rfc2822_leap_day() {
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz.with_ymd_and_hms(2028, 2, 29, 0, 0, 0).single().unwrap(); // Tuesday
        assert_eq!(to_rfc2822(&datetime), "Tue, 29 Feb 2028 00:00:00 +0000");
    }
}
