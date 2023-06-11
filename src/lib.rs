use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Default)]
pub struct Task {
    pub name: String,
    start_time: Option<DateTime<Utc>>,
    stop_time: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            start_time: None,
            stop_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Utc::now());
    }

    pub fn stop(&mut self) {
        self.stop_time = Some(Utc::now());
    }

    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn is_stoped(&self) -> bool {
        self.stop_time.is_some()
    }

    pub fn duration(&self) -> Duration {
        match self {
            Task {
                start_time: Some(start_time),
                stop_time: Some(stop_time),
                ..
            } => *stop_time - *start_time,
            _ => Duration::zero(),
        }
    }
}
