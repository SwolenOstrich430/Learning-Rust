#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod conn;
pub mod schema_reader;
pub mod column_types;
pub mod graphql_types;
pub mod db_column;
pub mod schema_writer;