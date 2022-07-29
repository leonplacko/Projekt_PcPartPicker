use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(&self, conn: &DBPooledConnection) -> Result<BuildSize, diesel::result::Error>;
    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<BuildSize>, diesel::result::Error>;
    fn update(
        &self,
        conn: &DBPooledConnection,
        other: BuildSize,
    ) -> Result<BuildSize, diesel::result::Error>;
    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
