use chrono::{DateTime, Utc};
pub trait LastUpdateTime {
    fn get_time(&self) -> DateTime<Utc>;
    fn get_time_formatted(&self) -> String {
        self.get_time().format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}
