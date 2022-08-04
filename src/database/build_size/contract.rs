use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(build: NewBuildSize, conn: &DBPooledConnection) -> Result<BuildSize, diesel::result::Error>;
    fn read(size_: String, conn: &DBPooledConnection) -> Result<Vec<BuildSize>, diesel::result::Error>;
    fn update(
        conn: &DBPooledConnection,
        other: BuildSize,
    ) -> Result<BuildSize, diesel::result::Error>;
    fn delete(mbid: Option<String>, csid: Option<String>, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
