use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn db_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}
