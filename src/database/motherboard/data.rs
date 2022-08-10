use serde::{Deserialize, Serialize};

use crate::database::schema::motherboards;

#[derive(Queryable, Debug, Serialize)]
pub struct Motherboard {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub ram_slots: i32,
    pub sata_slots: i32,
    pub nvme_slots: i32,
    pub price: f32,
}
#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "motherboards"]
pub struct NewMotherboard {
    pub name: String,
    pub manufacturer: String,
    pub ram_slots: i32,
    pub sata_slots: i32,
    pub nvme_slots: i32,
    pub price: f32,
}

#[derive(Deserialize)]
pub struct ExtendMotherboard {
    pub name: String,
    pub manufacturer: String,
    pub ram_slots: i32,
    pub sata_slots: i32,
    pub nvme_slots: i32,
    pub price: f32,
    pub socket: String,
    pub ram_type: String,
    pub build_size: String,
}

impl From<ExtendMotherboard> for NewMotherboard {
    fn from(exmb: ExtendMotherboard) -> Self {
        NewMotherboard {
            name: exmb.name.to_owned(),
            manufacturer: exmb.manufacturer.to_owned(),
            ram_slots: exmb.ram_slots.to_owned(),
            sata_slots: exmb.sata_slots.to_owned(),
            nvme_slots: exmb.nvme_slots.to_owned(),
            price: exmb.price.to_owned(),
        }
    }
}
