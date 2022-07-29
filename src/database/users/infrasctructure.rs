extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;


use super::contract::*;
use super::data::*;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl};
use r2d2::PooledConnection;
use schema::users;
use schema::users::dsl::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

impl CRUD for User {
    fn create(mut new_user: NewUser, conn: &DBPooledConnection) -> Result<User, diesel::result::Error> {
        let mut hasher = DefaultHasher::new();
        new_user.password.hash(&mut hasher);

        let hashed_pass = format!("{:x}", hasher.finish());

        new_user.password = hashed_pass;

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<User>, diesel::result::Error> {
        users.load::<User>(conn)
    }

    fn read_user(_username: String, conn: &DBPooledConnection) -> Result<User, diesel::result::Error>{
        users.filter(username.eq(_username)).first(conn)
    }

    fn update(
        &self,
        conn: &DBPooledConnection,
        other: User,
    ) -> Result<User, diesel::result::Error> {
        diesel::update(users.filter(username.eq(&self.username)))
            .set((username.eq(&other.username), password.eq(&other.password)))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(users.filter(username.eq(&self.username))).execute(conn)
    }
}
