extern crate db;
extern crate mysql;
use dotenv::dotenv;
use std::env;
use mysql::*;



fn main() {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("got connection {}", database_url);

    let mut db_conn: PooledConn = db::conn::get_connection(&database_url);

    let table_results: QueryResult = match db_conn.prep_exec("SHOW TABLES", ()) {
        Ok(results) => results, 
        Err(error) => panic!("couldn't get results {}", error)
    };

    for table in table_results {
        match table {
            Ok(table) => println!("{:?}", table), 
            _ => {}
        }
    }
    
}
