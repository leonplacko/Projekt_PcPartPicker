use crate::database::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub isadmin: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub isadmin: bool,
}
#[derive(Queryable, Deserialize)]
pub struct UserLog {
    pub username: String,
    pub password: String,
}
