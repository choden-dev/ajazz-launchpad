use rusqlite::{Error, Row};

trait Mapper<T> {
    fn map(&self, row: &Row) -> Result<T, Error>;
}

