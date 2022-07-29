//idu traitovi
use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub trait CRUD {
    fn create(user: NewUser, conn: &DBPooledConnection) -> Result<User, diesel::result::Error>;
    fn read_all(conn: &DBPooledConnection) -> Result<Vec<User>, diesel::result::Error>;
    fn read_user(u: String, conn: &DBPooledConnection) -> Result<User, diesel::result::Error>;
    fn update(&self, conn: &DBPooledConnection, other: User)
        -> Result<User, diesel::result::Error>;
    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error>;
}
