use serde::{Serialize, Deserialize};

use crate::database::schema::power_unit;

#[derive(Queryable, Debug)]
pub struct PowerUnit {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub power: i32,
    pub price: f32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "power_unit"]
pub struct NewPowerUnit {
    pub name: String,
    pub manufacturer: String,
    pub power: i32,
    pub price: f32,
}
