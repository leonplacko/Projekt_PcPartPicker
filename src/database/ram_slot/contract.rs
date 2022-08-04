use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(ramslot: NewRamSlot, conn: &DBPooledConnection) -> Result<RamSlot, diesel::result::Error>;
    fn read(slotype: String, conn: &DBPooledConnection) -> Result<Vec<RamSlot>, diesel::result::Error>;
    fn update(conn: &DBPooledConnection, other: RamSlot)
        -> Result<RamSlot, diesel::result::Error>;
    fn delete(mbid: Option<String>, ramid: Option<String>, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
