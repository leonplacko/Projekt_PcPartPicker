use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(newgpu: NewGPU, conn: &DBPooledConnection) -> Result<GPU, diesel::result::Error>;
    fn read_all(conn: &DBPooledConnection) -> Result<Vec<GPU>, diesel::result::Error>;
    fn update(conn: &DBPooledConnection, other: NewGPU) -> Result<GPU, diesel::result::Error>;
    fn delete(name: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
