use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(&self, conn: &DBPooledConnection) -> Result<Socket, diesel::result::Error>;
    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<Socket>, diesel::result::Error>;
    fn update(&self, conn: &DBPooledConnection, other: Socket) -> Result<Socket, diesel::result::Error>;
    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
