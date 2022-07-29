use serde::{Serialize, Deserialize};

use crate::database::schema::ram_slot;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "ram_slot"]
pub struct RamSlot {
    pub motherboard_id: String, // ovo je strani kljuc mozda je drugi tip
    pub ram_id: String,         // ovo je strani kljuc mozda je drugi tip
    pub type_: String,
}
