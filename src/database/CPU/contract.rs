use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(n_cpu: ExtendCPU, conn: &DBPooledConnection) -> Result<CPU, diesel::result::Error>;
    fn read_all(conn: &DBPooledConnection) -> Result<Vec<CPU>, diesel::result::Error>;
    fn update(conn: &DBPooledConnection, other: NewCPU) -> Result<CPU, diesel::result::Error>;
    fn delete(cpuname: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
