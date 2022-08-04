use serde::{Serialize, Deserialize};

use crate::database::schema::gpu;

#[derive(Queryable, Debug, Serialize)]
pub struct GPU {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub memory: i32,
    pub memory_type: String,
    pub tdp: i32,
    pub price: f32,
}

#[derive(Queryable, Deserialize, Insertable)]
#[table_name = "gpu"]
pub struct NewGPU {
    pub name: String,
    pub manufacturer: String,
    pub memory: i32,
    pub memory_type: String,
    pub tdp: i32,
    pub price: f32,
}
