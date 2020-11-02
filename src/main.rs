extern crate db;
extern crate mysql;
use dotenv::dotenv;
use std::env;
use mysql::*;
use db::db_column::Column;
use db::schema_reader;
use std::collections::HashMap;


fn main() {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let tables_query: &str = r"SELECT table_name, column_name, ordinal_position, is_nullable, data_type, column_key FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA LIKE 'Rust_Sample'";
    let db_conn: &mut PooledConn = &mut db::conn::get_connection(&database_url);
    
    let columns: Vec<Column> = schema_reader::get_columns(tables_query, db_conn);
    let table_map: HashMap<String, Vec<Column>> = schema_reader::get_table_map(columns);
}
