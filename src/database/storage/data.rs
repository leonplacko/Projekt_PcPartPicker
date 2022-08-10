use serde::{Deserialize, Serialize};

use crate::database::schema::storages;

#[derive(Queryable, Debug, Serialize)]
pub struct Storage {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub capacity: i32,
    pub speed: i32,
    pub type_: String,
    pub price: f32,
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "storages"]
pub struct NewStorage {
    pub name: String,
    pub manufacturer: String,
    pub capacity: i32,
    pub speed: i32,
    pub type_: String,
    pub price: f32,
}
#[derive(Deserialize)]
pub struct ExtendStorage {
    pub name: String,
    pub manufacturer: String,
    pub capacity: i32,
    pub speed: i32,
    pub type_: String,
    pub price: f32,
    pub slot: String,
}

impl From<ExtendStorage> for NewStorage {
    fn from(exstor: ExtendStorage) -> Self {
        NewStorage {
            name: exstor.name.to_owned(),
            manufacturer: exstor.manufacturer.to_owned(),
            capacity: exstor.capacity.to_owned(),
            speed: exstor.speed.to_owned(),
            type_: exstor.type_.to_owned(),
            price: exstor.price.to_owned(),
        }
    }
}
