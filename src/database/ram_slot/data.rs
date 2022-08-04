use serde::{Serialize, Deserialize};

use crate::database::schema::ram_slot;

#[derive(Queryable, Debug, Serialize)]
pub struct RamSlot {
    pub id: String,
    pub motherboard_id: Option<String>, // ovo je strani kljuc mozda je drugi tip
    pub ram_id: Option<String>,         // ovo je strani kljuc mozda je drugi tip
    pub type_: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "ram_slot"]
pub struct NewRamSlot {
    pub motherboard_id: Option<String>, // ovo je strani kljuc mozda je drugi tip
    pub ram_id: Option<String>,         // ovo je strani kljuc mozda je drugi tip
    pub type_: String,
}
