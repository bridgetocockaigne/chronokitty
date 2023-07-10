use chrono::{DateTime, Duration, Utc};

#[derive(Default, Debug)]
pub struct Task {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub labels: Vec<String>,
}

impl Task {
    pub fn start(&mut self) {
        self.start_date = Some(Utc::now());
    }

    pub fn stop(&mut self) {
        self.end_date = Some(Utc::now());
    }

    pub fn duration(&mut self) -> Duration {
        match self {
            _ => Duration::zero(),
        }
    }
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Task {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            labels: self.labels.to_vec(),
            start_date: self.start_date,
            end_date: self.end_date,
        }
    }
}
