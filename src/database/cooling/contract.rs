use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(
        cool: NewCooling,
        conn: &DBPooledConnection,
    ) -> Result<Cooling, diesel::result::Error>;
    fn read_all(conn: &DBPooledConnection) -> Result<Vec<Cooling>, diesel::result::Error>;
    fn update(
        conn: &DBPooledConnection,
        other: NewCooling,
    ) -> Result<Cooling, diesel::result::Error>;
    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
