use serde::{Serialize, Deserialize};

use crate::database::schema::pc_case;

#[derive(Queryable, Debug, Serialize)]
pub struct Case {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub dimensions: String,
    pub ventilators: i32,
    pub available_vent: i32,
    pub price: f32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "pc_case"]
pub struct NewCase {
    pub name: String,
    pub manufacturer: String,
    pub dimensions: String,
    pub ventilators: i32,
    pub available_vent: i32,
    pub price: f32,
}
