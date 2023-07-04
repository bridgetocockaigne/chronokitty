use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::{migrate::MigrateError, SqlitePool};

use crate::task::Task;

const DEFAULT_CONNECTION_STRING: &str = "sqlite::memory:";

#[derive(Debug)]
pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn save(&self, task: &Task) -> Result<Task, Error> {
        let mut connection = self.pool.acquire().await?;

        let id = sqlx::query("INSERT INTO tasks(name, description, labels, start_date, end_date) values(?, ?, ?, ?, ?)")
            .bind(&task.name)
            .bind(&task.description)
            .bind(json!(&task.labels).to_string())
            .bind(format_date(&task.start_date))
            .bind(format_date(&task.end_date))
            .execute(&mut *connection).await?.last_insert_rowid();

        Ok(Task {
            id: Some(id),
            ..task.clone()
        })
    }
}

#[derive(Default)]
pub struct StorageBuilder<'a> {
    connection_string: Option<&'a str>,
}

fn format_date(date: &Option<DateTime<Utc>>) -> String {
    match date {
        Some(date) => format!("{}", date.format("%+")),
        None => String::from(""),
    }
}

impl<'a> StorageBuilder<'a> {
    pub async fn build(self) -> Result<Storage, Error> {
        let url = self.connection_string.unwrap_or(DEFAULT_CONNECTION_STRING);
        let pool = SqlitePool::connect(url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Storage { pool })
    }
}

pub enum Error {
    MigrateError(),
    OperationError(String),
    GenericError(),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OperationError(e) => write!(f, "OperationError({e})"),
            Error::MigrateError() => write!(f, "MigrateError()"),
            Error::GenericError() => write!(f, "GenericError()"),
        }
    }
}

impl From<MigrateError> for Error {
    fn from(_: MigrateError) -> Self {
        Error::MigrateError()
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(e) => Self::OperationError(e.to_string()),
            _ => Self::GenericError(),
        }
    }
}
