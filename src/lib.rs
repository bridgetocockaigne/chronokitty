use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Default)]
pub struct Task {
    pub name: String,
    pub start_time: DateTime<Utc>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            start_time: Utc::now(),
        }
    }

    pub fn elapsed_time(&self) -> Duration {
        Utc::now() - self.start_time
    }
}
