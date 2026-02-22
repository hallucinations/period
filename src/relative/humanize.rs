use chrono::{DateTime, Local};

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
///
/// Note: the `< 90 s` bucket uses the article form (`"a minute ago"`).
/// Values in the `< 45 min` bucket that round to `n = 1` produce `"1 minute ago"`,
/// so the transition at 90 s goes `"a minute ago"` → `"1 minute ago"` → `"2 minutes ago"`.
///
/// Note: `"yesterday"` / `"tomorrow"` are triggered by elapsed seconds (22–35 h),
/// not by calendar-day boundaries. A datetime 23 hours ago may return `"yesterday"`
/// even if it falls on the current calendar day.
///
/// Months are approximated as 30 days for bucketing purposes.
#[inline]
#[must_use]
pub fn humanize(datetime: DateTime<Local>) -> String {
    humanize_impl(datetime, Local::now())
}

fn humanize_impl(datetime: DateTime<Local>, now: DateTime<Local>) -> String {
    const MINUTE: i64 = 60;
    const HOUR: i64 = 3_600;
    const DAY: i64 = 86_400;
    const MONTH: i64 = 30 * DAY;
    const YEAR: i64 = 365 * DAY;

    let secs = now.signed_duration_since(datetime).num_seconds();
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
    use crate::relative::functions::{days_ago, hours_from_now};
    use chrono::Duration;

    /// Call `humanize_impl` with a single captured `now`, so both the reference
    /// datetime and the comparison point are derived from the same instant.
    /// This eliminates any wall-clock drift between test setup and assertion,
    /// removing the need for ad-hoc time buffers in future-facing tests.
    fn h_past(secs: i64) -> String {
        let now = Local::now();
        humanize_impl(now - Duration::seconds(secs), now)
    }

    fn h_future(secs: i64) -> String {
        let now = Local::now();
        humanize_impl(now + Duration::seconds(secs), now)
    }

    #[test]
    fn test_humanize_just_now_past() {
        assert_eq!(h_past(10), "just now");
    }

    #[test]
    fn test_humanize_just_now_future() {
        assert_eq!(h_future(10), "just now");
    }

    #[test]
    fn test_humanize_a_minute_ago() {
        assert_eq!(h_past(60), "a minute ago");
    }

    #[test]
    fn test_humanize_in_a_minute() {
        assert_eq!(h_future(60), "in a minute");
    }

    #[test]
    fn test_humanize_minutes_ago() {
        assert_eq!(h_past(5 * 60), "5 minutes ago");
    }

    #[test]
    fn test_humanize_in_minutes() {
        assert_eq!(h_future(5 * 60), "in 5 minutes");
    }

    #[test]
    fn test_humanize_1_minute_singular_ago() {
        // 95 s -> n = 1 -> singular
        assert_eq!(h_past(95), "1 minute ago");
    }

    #[test]
    fn test_humanize_an_hour_ago() {
        assert_eq!(h_past(60 * 60), "an hour ago");
    }

    #[test]
    fn test_humanize_in_an_hour() {
        assert_eq!(h_future(60 * 60), "in an hour");
    }

    #[test]
    fn test_humanize_hours_ago() {
        assert_eq!(h_past(5 * 3600), "5 hours ago");
    }

    #[test]
    fn test_humanize_in_hours() {
        assert_eq!(h_future(5 * 3600), "in 5 hours");
    }

    #[test]
    fn test_humanize_yesterday() {
        assert_eq!(h_past(24 * 3600), "yesterday");
    }

    #[test]
    fn test_humanize_tomorrow() {
        assert_eq!(h_future(24 * 3600), "tomorrow");
    }

    #[test]
    fn test_humanize_days_ago() {
        assert_eq!(h_past(5 * 86_400), "5 days ago");
    }

    #[test]
    fn test_humanize_in_days() {
        assert_eq!(h_future(5 * 86_400), "in 5 days");
    }

    #[test]
    fn test_humanize_a_month_ago() {
        assert_eq!(h_past(30 * 86_400), "a month ago");
    }

    #[test]
    fn test_humanize_in_a_month() {
        assert_eq!(h_future(30 * 86_400), "in a month");
    }

    #[test]
    fn test_humanize_months_ago() {
        assert_eq!(h_past(3 * 30 * 86_400), "3 months ago");
    }

    #[test]
    fn test_humanize_in_months() {
        assert_eq!(h_future(3 * 30 * 86_400), "in 3 months");
    }

    #[test]
    fn test_humanize_a_year_ago() {
        // 13 months past -- inside "< 18 months" bucket
        assert_eq!(h_past(13 * 30 * 86_400), "a year ago");
    }

    #[test]
    fn test_humanize_in_a_year() {
        assert_eq!(h_future(13 * 30 * 86_400), "in a year");
    }

    #[test]
    fn test_humanize_years_ago() {
        assert_eq!(h_past(3 * 365 * 86_400), "3 years ago");
    }

    #[test]
    fn test_humanize_in_years() {
        assert_eq!(h_future(3 * 365 * 86_400), "in 3 years");
    }

    #[test]
    fn test_humanize_in_1_minute_singular() {
        // 95 s -> n = 1 -> singular future form
        assert_eq!(h_future(95), "in 1 minute");
    }

    #[test]
    fn test_humanize_1_hour_singular_ago() {
        // 5401 s (approx 90 min) -> hours bucket, n = 1
        assert_eq!(h_past(5401), "1 hour ago");
    }

    #[test]
    fn test_humanize_in_1_hour_singular() {
        assert_eq!(h_future(5401), "in 1 hour");
    }

    #[test]
    fn test_humanize_1_day_singular_ago() {
        // 37 h -> days bucket, n = 1
        assert_eq!(h_past(37 * 3_600), "1 day ago");
    }

    #[test]
    fn test_humanize_in_1_day_singular() {
        assert_eq!(h_future(37 * 3_600), "in 1 day");
    }

    #[test]
    fn test_humanize_1_month_singular_ago() {
        // 46 days -> months bucket, n = 1
        assert_eq!(h_past(46 * 86_400), "1 month ago");
    }

    #[test]
    fn test_humanize_in_1_month_singular() {
        assert_eq!(h_future(46 * 86_400), "in 1 month");
    }

    #[test]
    fn test_humanize_1_year_singular_ago() {
        // 19 x 30 days -> years bucket, n = 1
        assert_eq!(h_past(19 * 30 * 86_400), "1 year ago");
    }

    #[test]
    fn test_humanize_in_1_year_singular() {
        assert_eq!(h_future(19 * 30 * 86_400), "in 1 year");
    }

    // -- exact threshold boundaries -------------------------------------------

    #[test]
    fn test_humanize_exactly_30s_past_is_a_minute_ago() {
        assert_eq!(h_past(30), "a minute ago");
    }

    #[test]
    fn test_humanize_exactly_30s_future_is_in_a_minute() {
        assert_eq!(h_future(30), "in a minute");
    }

    #[test]
    fn test_humanize_89s_past_is_a_minute_ago() {
        assert_eq!(h_past(89), "a minute ago");
    }

    #[test]
    fn test_humanize_44_minutes_past() {
        assert_eq!(h_past(44 * 60), "44 minutes ago");
    }

    #[test]
    fn test_humanize_exactly_45_minutes_past_is_an_hour_ago() {
        assert_eq!(h_past(45 * 60), "an hour ago");
    }

    #[test]
    fn test_humanize_exactly_45_minutes_future_is_in_an_hour() {
        assert_eq!(h_future(45 * 60), "in an hour");
    }

    #[test]
    fn test_humanize_89_minutes_past_is_an_hour_ago() {
        assert_eq!(h_past(89 * 60), "an hour ago");
    }

    #[test]
    fn test_humanize_21_hours_past() {
        assert_eq!(h_past(21 * 3_600), "21 hours ago");
    }

    #[test]
    fn test_humanize_exactly_22_hours_past_is_yesterday() {
        assert_eq!(h_past(22 * 3_600), "yesterday");
    }

    #[test]
    fn test_humanize_exactly_22_hours_future_is_tomorrow() {
        assert_eq!(h_future(22 * 3_600), "tomorrow");
    }

    #[test]
    fn test_humanize_35_hours_past_is_yesterday() {
        assert_eq!(h_past(35 * 3_600), "yesterday");
    }

    #[test]
    fn test_humanize_exactly_36_hours_past_is_1_day_ago() {
        assert_eq!(h_past(36 * 3_600), "1 day ago");
    }

    #[test]
    fn test_humanize_24_days_past() {
        assert_eq!(h_past(24 * 86_400), "24 days ago");
    }

    #[test]
    fn test_humanize_exactly_25_days_past_is_a_month_ago() {
        assert_eq!(h_past(25 * 86_400), "a month ago");
    }

    #[test]
    fn test_humanize_exactly_25_days_future_is_in_a_month() {
        assert_eq!(h_future(25 * 86_400), "in a month");
    }

    #[test]
    fn test_humanize_exactly_45_days_past_is_months_ago() {
        // 45 * 86400 / (30 * 86400) = 1 -> "1 month ago"
        assert_eq!(h_past(45 * 86_400), "1 month ago");
    }

    #[test]
    fn test_humanize_9_months_past() {
        assert_eq!(h_past(9 * 30 * 86_400), "9 months ago");
    }

    #[test]
    fn test_humanize_exactly_10_months_past_is_a_year_ago() {
        assert_eq!(h_past(10 * 30 * 86_400), "a year ago");
    }

    #[test]
    fn test_humanize_exactly_10_months_future_is_in_a_year() {
        assert_eq!(h_future(10 * 30 * 86_400), "in a year");
    }

    #[test]
    fn test_humanize_exactly_18_months_past_is_years_ago() {
        // 18 * 30 * 86400 / (365 * 86400) ≈ 1 -> "1 year ago"
        assert_eq!(h_past(18 * 30 * 86_400), "1 year ago");
    }

    #[test]
    fn test_humanize_exactly_18_months_future_is_in_years() {
        assert_eq!(h_future(18 * 30 * 86_400), "in 1 year");
    }

    #[test]
    fn test_humanize_now_is_just_now() {
        let now = Local::now();
        assert_eq!(humanize_impl(now, now), "just now");
    }

    // -- humanize with Relative-derived datetime ------------------------------

    #[test]
    fn test_humanize_with_days_ago_relative() {
        let dt: DateTime<Local> = days_ago(3).unwrap().into();
        assert_eq!(humanize(dt), "3 days ago");
    }

    #[test]
    fn test_humanize_with_hours_from_now_relative() {
        let r = hours_from_now(5).unwrap();
        let result = humanize(r.into());
        assert!(result.starts_with("in "), "expected 'in …', got: {result}");
    }

    // -- extreme values -------------------------------------------------------

    #[test]
    fn test_humanize_extreme_past_is_n_years_ago() {
        // Use a large but representable second count (1000 years ≈ 31_557_600_000 s).
        // Exercises the ">= 18 months" else branch and saturating_abs on large i64 values.
        let secs: i64 = 1_000 * 365 * 86_400;
        let result = h_past(secs);
        assert!(
            result.ends_with("years ago"),
            "expected 'N years ago', got: {result}"
        );
    }

    #[test]
    fn test_humanize_extreme_future_is_in_n_years() {
        let secs: i64 = 1_000 * 365 * 86_400;
        let result = h_future(secs);
        assert!(
            result.starts_with("in ") && result.ends_with("years"),
            "expected 'in N years', got: {result}"
        );
    }
}
