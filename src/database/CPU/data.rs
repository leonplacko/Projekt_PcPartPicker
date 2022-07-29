use serde::{Serialize, Deserialize};

use crate::database::schema::cpu;

#[derive(Queryable, Debug)]
pub struct CPU {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub cores: i32,
    pub cache: String,
    pub speed: f32,
    pub tdp: i32,
    pub price: f32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "cpu"]
pub struct NewCPU {
    pub name: String,
    pub manufacturer: String,
    pub cores: i32,
    pub cache: String,
    pub speed: f32,
    pub tdp: i32,
    pub price: f32,
}
