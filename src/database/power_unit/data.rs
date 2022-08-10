use serde::{Deserialize, Serialize};

use crate::database::schema::power_unit;

#[derive(Queryable, Debug, Serialize)]
pub struct PowerUnit {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub power: i32,
    pub price: f32,
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "power_unit"]
pub struct NewPowerUnit {
    pub name: String,
    pub manufacturer: String,
    pub power: i32,
    pub price: f32,
}
