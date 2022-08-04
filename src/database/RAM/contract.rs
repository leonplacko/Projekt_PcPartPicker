use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(ram_: ExtendRAM, conn: &DBPooledConnection) -> Result<RAM, diesel::result::Error>;
    fn read_all(conn: &DBPooledConnection) -> Result<Vec<RAM>, diesel::result::Error>;
    fn update(conn: &DBPooledConnection, other: NewRAM) -> Result<RAM, diesel::result::Error>;
    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
