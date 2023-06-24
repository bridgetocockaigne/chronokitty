use std::{collections::HashSet, fmt::Display};

use sqlx::{Connection, SqliteConnection};

use crate::task::Task;

#[derive(Debug, Default)]
pub struct Storage {
    conn: Option<SqliteConnection>,
}

impl Storage {
    pub async fn store(&mut self, task: &Task) -> Result<Task, Error> {
        let connection_error = Error::ConnectionError(String::from("No connection extablished"));

        let id = sqlx::query("INSERT INTO tasks (name) VALUES (?1)")
            .bind(&task.name)
            .execute(self.conn.as_mut().ok_or(connection_error)?)
            .await?
            .last_insert_rowid();

        Ok(Task {
            id: Some(id),
            start_time: None,
            stop_time: None,
            name: task.name.clone(),
            label: HashSet::new(),
        })
    }
}

#[derive(Default)]
pub struct StorageBuilder {
    connection: Option<String>,
}

impl StorageBuilder {
    pub async fn build(&self) -> Result<Storage, Error> {
        let connection = self
            .connection
            .clone()
            .unwrap_or(String::from("sqlite::memory:"));

        let conn = SqliteConnection::connect(&connection)
            .await
            .map_err(|_| Error::ConnectionError(connection))?;

        Ok(Storage { conn: Some(conn) })
    }

    pub fn with_connection_string(&mut self, connection_string: String) {
        self.connection = Some(connection_string);
    }
}

pub enum Error {
    ConnectionError(String),
    OperationError(String),
    GenericError(),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Configuration(_) => todo!(),
            sqlx::Error::Database(_) => todo!(),
            sqlx::Error::Io(_) => todo!(),
            sqlx::Error::Tls(_) => todo!(),
            sqlx::Error::Protocol(_) => todo!(),
            sqlx::Error::RowNotFound => todo!(),
            sqlx::Error::TypeNotFound { type_name } => todo!(),
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
            sqlx::Error::ColumnNotFound(_) => todo!(),
            sqlx::Error::ColumnDecode { index, source } => todo!(),
            sqlx::Error::Decode(_) => todo!(),
            sqlx::Error::PoolTimedOut => todo!(),
            sqlx::Error::PoolClosed => todo!(),
            sqlx::Error::WorkerCrashed => todo!(),
            sqlx::Error::Migrate(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionError(e) => write!(f, "ConnectionError: {}", e),
            Error::OperationError(_) => todo!(),
            Error::GenericError() => todo!(),
        }
    }
}
