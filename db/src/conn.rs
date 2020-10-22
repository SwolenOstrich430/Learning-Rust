use mysql::PooledConn;
use mysql::Pool;

pub fn get_connection(database_url: &String) -> PooledConn {
    let pool: Pool = Pool::new(database_url).unwrap();
    let db_conn: PooledConn = match pool.get_conn() {
        Ok(conn) => conn, 
        Err(error) => panic!("Could not create connection from url: {}, err: {}", database_url, error)
    };

    return db_conn;
}