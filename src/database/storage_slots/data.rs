use serde::{Serialize, Deserialize};

use crate::database::schema::storage_slots;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "storage_slots"]
pub struct STRGslots {
    pub motherboard_id: String, // ovo je strani kljuc mozda je drugi tip
    pub storage_id: String,     //ovo je strani kljuc mozda je drugi tip
    pub slot: String,
}
