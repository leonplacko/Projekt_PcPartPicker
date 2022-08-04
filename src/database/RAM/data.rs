use serde::{Serialize, Deserialize};

use crate::database::schema::ram;

#[derive(Queryable, Debug, Serialize)]
pub struct RAM {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub speed: i32,
    pub capacity: i32,
    pub price: f32,
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "ram"]
pub struct NewRAM {
    pub name: String,
    pub manufacturer: String,
    pub speed: i32,
    pub capacity: i32,
    pub price: f32,
}
#[derive(Deserialize)]
pub struct ExtendRAM{
    pub name: String,
    pub manufacturer: String,
    pub speed: i32,
    pub capacity: i32,
    pub price: f32,
    pub slot: String,
}

impl From<ExtendRAM> for NewRAM{
    fn from(exram: ExtendRAM) -> Self {
        NewRAM { 
            name: exram.name.to_owned(), 
            manufacturer: exram.manufacturer.to_owned(), 
            speed: exram.speed.to_owned(), 
            capacity: exram.capacity.to_owned(), 
            price: exram.price.to_owned(),
        }
    }
}
