use chrono::{Datelike, NaiveDate};

/// Returns `true` if `date` falls on a Saturday or Sunday.
#[must_use]
#[inline]
pub fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
}

/// Returns `true` if `date` falls on a Monday through Friday.
#[must_use]
#[inline]
pub fn is_weekday(date: NaiveDate) -> bool {
    !is_weekend(date)
}

/// Returns the 1-based day of the year (1 = January 1, 365 or 366 = December 31).
#[must_use]
#[inline]
pub fn day_of_year(date: NaiveDate) -> u32 {
    date.ordinal()
}

/// Returns the number of days in the month of `date` (28–31).
///
/// # Panics
///
/// Never panics for any valid [`NaiveDate`].
#[must_use]
#[inline]
pub fn days_in_month(date: NaiveDate) -> u32 {
    // The first day of the next month minus one day gives the last day of the
    // current month; its `day()` is the count of days in the month.
    let (year, month) = if date.month() == 12 {
        (date.year() + 1, 1)
    } else {
        (date.year(), date.month() + 1)
    };
    NaiveDate::from_ymd_opt(year, month, 1)
        .expect("valid next-month date")
        .pred_opt()
        .expect("valid predecessor")
        .day()
}

/// Returns the ISO 8601 week number (1–53) for `date`.
///
/// Weeks start on Monday. The first week of the year is the week containing
/// the first Thursday (ISO 8601 definition). Week numbers can be 1–53.
#[must_use]
#[inline]
pub fn week_of_year(date: NaiveDate) -> u32 {
    date.iso_week().week()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    // -- is_weekend / is_weekday ----------------------------------------------

    #[test]
    fn test_is_weekend_saturday() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 21).unwrap(); // Saturday
        assert!(is_weekend(date));
        assert!(!is_weekday(date));
    }

    #[test]
    fn test_is_weekend_sunday() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap(); // Sunday
        assert!(is_weekend(date));
        assert!(!is_weekday(date));
    }

    #[test]
    fn test_is_weekday_monday() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 23).unwrap(); // Monday
        assert!(is_weekday(date));
        assert!(!is_weekend(date));
    }

    #[test]
    fn test_is_weekday_friday() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(); // Friday
        assert!(is_weekday(date));
        assert!(!is_weekend(date));
    }

    #[test]
    fn test_is_weekend_covers_all_days_of_week() {
        // 2026-02-23 is Monday; step through a full week
        let monday = NaiveDate::from_ymd_opt(2026, 2, 23).unwrap();
        let expected = [false, false, false, false, false, true, true]; // Mon–Sun
        for (i, &expected_weekend) in expected.iter().enumerate() {
            let date = monday + chrono::Duration::days(i as i64);
            assert_eq!(
                is_weekend(date),
                expected_weekend,
                "failed for {date} (offset {i})"
            );
        }
    }

    // -- day_of_year ----------------------------------------------------------

    #[test]
    fn test_day_of_year_jan_1() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        assert_eq!(day_of_year(date), 1);
    }

    #[test]
    fn test_day_of_year_dec_31_non_leap() {
        let date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        assert_eq!(day_of_year(date), 365);
    }

    #[test]
    fn test_day_of_year_dec_31_leap() {
        let date = NaiveDate::from_ymd_opt(2028, 12, 31).unwrap();
        assert_eq!(day_of_year(date), 366);
    }

    #[test]
    fn test_day_of_year_leap_day() {
        let date = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
        assert_eq!(day_of_year(date), 60);
    }

    #[test]
    fn test_day_of_year_mar_1_after_leap() {
        // March 1 in a leap year is day 61 (not 60 as in non-leap years)
        let date = NaiveDate::from_ymd_opt(2028, 3, 1).unwrap();
        assert_eq!(day_of_year(date), 61);
    }

    #[test]
    fn test_day_of_year_mar_1_non_leap() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        assert_eq!(day_of_year(date), 60);
    }

    // -- days_in_month --------------------------------------------------------

    #[test]
    fn test_days_in_month_january() {
        assert_eq!(
            days_in_month(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
            31
        );
    }

    #[test]
    fn test_days_in_month_february_non_leap() {
        assert_eq!(
            days_in_month(NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()),
            28
        );
    }

    #[test]
    fn test_days_in_month_february_leap_divisible_by_4() {
        // Ordinary leap year: divisible by 4
        assert_eq!(
            days_in_month(NaiveDate::from_ymd_opt(2028, 2, 1).unwrap()),
            29
        );
    }

    #[test]
    fn test_days_in_month_february_century_non_leap() {
        // Century years are NOT leap years unless also divisible by 400
        assert_eq!(
            days_in_month(NaiveDate::from_ymd_opt(2100, 2, 1).unwrap()),
            28
        );
    }

    #[test]
    fn test_days_in_month_february_400_year_leap() {
        // Divisible by 400 IS a leap year
        assert_eq!(
            days_in_month(NaiveDate::from_ymd_opt(2000, 2, 1).unwrap()),
            29
        );
    }

    #[test]
    fn test_days_in_month_april() {
        assert_eq!(
            days_in_month(NaiveDate::from_ymd_opt(2026, 4, 1).unwrap()),
            30
        );
    }

    #[test]
    fn test_days_in_month_december() {
        assert_eq!(
            days_in_month(NaiveDate::from_ymd_opt(2026, 12, 15).unwrap()),
            31
        );
    }

    #[test]
    fn test_days_in_month_result_same_regardless_of_day_input() {
        // Any day within the month should give the same count
        let d1 = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let d2 = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
        let d3 = NaiveDate::from_ymd_opt(2026, 3, 31).unwrap();
        assert_eq!(days_in_month(d1), 31);
        assert_eq!(days_in_month(d2), 31);
        assert_eq!(days_in_month(d3), 31);
    }

    // -- week_of_year ---------------------------------------------------------

    #[test]
    fn test_week_of_year_jan_1_2026() {
        // 2026-01-01 is a Thursday — belongs to week 1
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        assert_eq!(week_of_year(date), 1);
    }

    #[test]
    fn test_week_of_year_jan_1_2016() {
        // 2016-01-01 is a Friday — also week 53 of 2015 or week 1 of 2016?
        // ISO: belongs to week 53 of 2015 (last week of prior year)
        let date = NaiveDate::from_ymd_opt(2016, 1, 1).unwrap();
        assert_eq!(week_of_year(date), 53);
    }

    #[test]
    fn test_week_of_year_feb_22_2026() {
        // 2026-02-22 is a Sunday in week 8
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(week_of_year(date), 8);
    }

    #[test]
    fn test_week_of_year_dec_28() {
        // 2026-12-28 is a Monday — always in the last week of its ISO year
        let date = NaiveDate::from_ymd_opt(2026, 12, 28).unwrap();
        assert_eq!(week_of_year(date), 53);
    }

    #[test]
    fn test_week_of_year_dec_31_2026() {
        // 2026-12-31 Thursday — week 53
        let date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        assert_eq!(week_of_year(date), 53);
    }
}
