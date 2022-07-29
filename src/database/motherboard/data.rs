use serde::{Deserialize, Serialize};

use crate::database::schema::motherboards;

#[derive(Queryable, Debug)]
pub struct Motherboard {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub ram_slots: i32,
    pub sata_slots: i32,
    pub nvme_slots: i32,
    pub price: f32,
}
#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "motherboards"]
pub struct NewMotherboard {
    pub name: String,
    pub manufacturer: String,
    pub ram_slots: i32,
    pub sata_slots: i32,
    pub nvme_slots: i32,
    pub price: f32,
}
