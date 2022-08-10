use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(
        storslot: NewSTRGslots,
        conn: &DBPooledConnection,
    ) -> Result<STRGslots, diesel::result::Error>;
    fn read(
        slott: String,
        conn: &DBPooledConnection,
    ) -> Result<Vec<STRGslots>, diesel::result::Error>;
    fn update(
        conn: &DBPooledConnection,
        other: STRGslots,
    ) -> Result<STRGslots, diesel::result::Error>;
    fn delete(
        mbid: Option<String>,
        storid: Option<String>,
        conn: &DBPooledConnection,
    ) -> Result<usize, diesel::result::Error>;
}
