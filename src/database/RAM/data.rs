use serde::{Serialize, Deserialize};

use crate::database::schema::ram;

#[derive(Queryable, Debug)]
pub struct RAM {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub speed: i32,
    pub capacity: i32,
    pub price: f32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "ram"]
pub struct NewRAM {
    pub name: String,
    pub manufacturer: String,
    pub speed: i32,
    pub capacity: i32,
    pub price: f32,
}
