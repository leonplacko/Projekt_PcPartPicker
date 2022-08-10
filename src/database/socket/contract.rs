use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait CRUD {
    fn create(sock: NewSocket, conn: &DBPooledConnection) -> Result<Socket, diesel::result::Error>;
    fn read(sockt: String, conn: &DBPooledConnection)
        -> Result<Vec<Socket>, diesel::result::Error>;
    fn update(conn: &DBPooledConnection, other: Socket) -> Result<Socket, diesel::result::Error>;
    fn delete(
        mbid: Option<String>,
        cpuid: Option<String>,
        conn: &DBPooledConnection,
    ) -> Result<usize, diesel::result::Error>;
}
