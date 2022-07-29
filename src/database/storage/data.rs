use serde::{Deserialize, Serialize};

use crate::database::schema::storages;

#[derive(Queryable, Debug)]
pub struct Storage {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub capacity: i32,
    pub speed: i32,
    pub type_: String,
    pub price: f32,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "storages"]
pub struct NewStorage {
    pub name: String,
    pub manufacturer: String,
    pub capacity: i32,
    pub speed: i32,
    pub type_: String,
    pub price: f32,
}
