use chrono::{DateTime, Utc};
pub trait LastUpdateTime {
    fn get_time(&self) -> DateTime<Utc>;
    fn get_time_formatted(&self) -> String {
        self.get_time().format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_time_format() {
        let time = DateTime::from_timestamp(0, 0).unwrap();
        struct Test {
            time: DateTime<Utc>,
        }
        impl LastUpdateTime for Test {
            fn get_time(&self) -> DateTime<Utc> {
                self.time
            }
        }
        let formatted = Test { time }.get_time_formatted();
        assert_eq!(formatted, "1970-01-01 00:00:00 UTC");
    }
}
