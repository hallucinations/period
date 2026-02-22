use chrono::{Duration, Local, NaiveDate};

pub fn days_ago(days: i64) -> NaiveDate {
    Local::now().date_naive() - Duration::days(days)
}

pub fn days_from_now(days: i64) -> NaiveDate {
    Local::now().date_naive() + Duration::days(days)
}

pub fn weeks_ago(weeks: i64) -> NaiveDate {
    Local::now().date_naive() - Duration::weeks(weeks)
}

pub fn weeks_from_now(weeks: i64) -> NaiveDate {
    Local::now().date_naive() + Duration::weeks(weeks)
}

pub fn yesterday() -> NaiveDate {
    days_ago(1)
}

pub fn tomorrow() -> NaiveDate {
    days_from_now(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_days_ago_returns_correct_date() {
        let date = days_ago(3);
        let expected = Local::now().date_naive() - Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_ago_with_zero_days() {
        let date = days_ago(0);
        let expected = Local::now().date_naive();
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_from_now_returns_correct_date() {
        let date = days_from_now(3);
        let expected = Local::now().date_naive() + Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_ago_returns_correct_date() {
        let date = weeks_ago(2);
        let expected = Local::now().date_naive() - Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_from_now_returns_correct_date() {
        let date = weeks_from_now(2);
        let expected = Local::now().date_naive() + Duration::weeks(2);
        assert_eq!(date, expected);
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
}
