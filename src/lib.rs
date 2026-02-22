use chrono::{Local, NaiveDate, DateTime, Duration};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

pub fn yesterday() -> NaiveDate {
    Local::now().date_naive() - Duration::days(1)
}

pub fn tomorrow() -> NaiveDate {
    Local::now().date_naive() + Duration::days(1)
}

pub fn days_ago(days: i64) -> NaiveDate {
    Local::now().date_naive() - Duration::days(days)
}

pub fn days_from_now(days: i64) -> NaiveDate {
    Local::now().date_naive() + Duration::days(days)
}

pub fn now() -> DateTime<Local> {
    Local::now()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_today_returns_current_date() {
        let date = today();
        let expected = Local::now().date_naive();
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

    #[test]
    fn test_days_ago_returns_correct_date() {
        let date = days_ago(3);
        let expected = Local::now().date_naive() - Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_from_now_returns_correct_date() {
        let date = days_from_now(3);
        let expected = Local::now().date_naive() + Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_now_returns_current_datetime() {
        let before = Local::now();
        let result = now();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }
}
