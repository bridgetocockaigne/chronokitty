use std::fmt::Display;

use sqlx::{Connection, SqliteConnection};

use crate::task::Task;

#[derive(Debug, Default)]
pub struct Storage {
    conn: Option<SqliteConnection>,
}

impl Storage {
    pub fn store(&self, task: Task) -> Result<Task, Error> {
        let connection_error = Error::ConnectionError(String::from("No connection extablished"));
        let conn = self.conn.as_ref().ok_or(connection_error)?;

        todo!();
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
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionError(e) => write!(f, "ConnectionError: {}", e),
        }
    }
}
