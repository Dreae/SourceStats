use diesel::PgConnection;
use r2d2::{Pool as r2d2Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

pub struct Pool {
    pool: r2d2Pool<ConnectionManager<PgConnection>>,
}

impl Pool {
    pub fn new(max_connections: usize, url: &str) -> Result<Pool, r2d2::Error> {
        let manager = ConnectionManager::new(url);
        Ok(Pool {
            pool: r2d2Pool::builder().max_size(max_connections as u32).build(manager)?
        })
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get a connection from the DB pool")
    }
}

impl Clone for Pool {
    fn clone(&self) -> Self {
        Pool {
            pool: self.pool.clone()
        }
    }
}