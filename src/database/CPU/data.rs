use serde::{Serialize, Deserialize};

use crate::database::schema::cpu;

#[derive(Queryable, Debug, Serialize)]
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

#[derive(Queryable, Insertable, Deserialize)]
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
#[derive(Deserialize)]
pub struct ExtendCPU{
    pub name: String,
    pub manufacturer: String,
    pub cores: i32,
    pub cache: String,
    pub speed: f32,
    pub tdp: i32,
    pub price: f32,
    pub socket: String, 
}

impl From<ExtendCPU> for NewCPU{
    fn from(excpu: ExtendCPU) -> Self {
        NewCPU{
            name: excpu.name.to_owned(),
            manufacturer: excpu.manufacturer.to_owned(),
            cores: excpu.cores.to_owned(),
            cache: excpu.cache.to_owned(),
            speed: excpu.speed.to_owned(),
            tdp: excpu.tdp.to_owned(),
            price: excpu.price.to_owned(),
        }
    }
}
