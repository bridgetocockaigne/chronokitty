use chrono::{DateTime, Utc};

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
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Task {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            labels: self.labels.iter().cloned().collect(),
            start_date: self.start_date,
            end_date: self.end_date,
        }
    }
}
