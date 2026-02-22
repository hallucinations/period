use chrono::{DateTime, Local, NaiveDate};

pub fn now() -> DateTime<Local> {
    Local::now()
}

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_now_returns_current_datetime() {
        let before = Local::now();
        let result = now();
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_today_returns_current_date() {
        let date = today();
        let expected = Local::now().date_naive();
        assert_eq!(date, expected);
    }
}
