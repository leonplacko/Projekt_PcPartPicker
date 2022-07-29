use serde::{Serialize, Deserialize};

use crate::database::schema::cooling;

#[derive(Queryable, Debug)]
pub struct Cooling {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub ventilators: Option<i32>,
    pub cpu_ventilator: Option<bool>,
    pub price: f32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "cooling"]
pub struct NewCooling {
    pub name: String,
    pub manufacturer: String,
    pub ventilators: Option<i32>,
    pub cpu_ventilator: Option<bool>,
    pub price: f32,
}
