use rusqlite::{Connection, Result as SqliteResult};
const DB_PATH: &str = "./ajazz_launchpad_db.db3";
pub struct SqLite {
    connection: Option<Connection>,
}

impl SqLite {
    pub fn new(persisted: bool) -> SqliteResult<SqLite> {
        let connection = match persisted {
            true => Connection::open(DB_PATH)?,
            false => Connection::open_in_memory()?,
        };
        Ok(SqLite {
            connection: Some(connection),
        })
    }
    pub fn close(&mut self) -> SqliteResult<()> {
        if let Some(connection) = self.connection.take() {
            connection.close().map_err(|(_, err)| err)?;
        }
        Ok(())
    }
    pub fn connection(&self) -> Option<&Connection> {
        self.connection.as_ref()
    }
}
impl Drop for SqLite {
    fn drop(&mut self) {
        if self.connection.is_some() {
            let _ = self.close();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Person {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    #[test]
    /// This test just makes sure our wrapper is capable of handling the
    /// default [`rusqlite` example](https://docs.rs/rusqlite/latest/rusqlite/)
    fn functioning_database() {
        let db = SqLite::new(false).unwrap();
        let connection = db.connection().unwrap();

        connection
            .execute(
                "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
                (),
            )
            .unwrap();

        let me = Person {
            id: 0,
            name: "Steven".to_string(),
            data: None,
        };
        connection
            .execute(
                "INSERT INTO person (name, data) VALUES (?1, ?2)",
                (&me.name, &me.data),
            )
            .unwrap();

        let mut stmt = connection
            .prepare("SELECT id, name, data FROM person")
            .unwrap();

        let person_iter = stmt
            .query_map([], |row| {
                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    data: row.get(2)?,
                })
            })
            .unwrap();

        for person in person_iter {
            let person = person.unwrap();

            assert_eq!(person.id, 1);
            assert_eq!(person.name, me.name);
            assert_eq!(person.data, me.data);
        }
    }
}
