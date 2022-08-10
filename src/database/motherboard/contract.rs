use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(
        mb: ExtendMotherboard,
        conn: &DBPooledConnection,
    ) -> Result<Motherboard, diesel::result::Error>;
    fn read(conn: &DBPooledConnection) -> Result<Vec<Motherboard>, diesel::result::Error>;
    fn update(
        conn: &DBPooledConnection,
        other: NewMotherboard,
    ) -> Result<Motherboard, diesel::result::Error>;
    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
