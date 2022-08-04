use serde::{Serialize, Deserialize};

use crate::database::schema::storage_slots;

#[derive(Queryable, Debug, Serialize)]
pub struct STRGslots {
    pub id: String,
    pub motherboard_id: Option<String>, // ovo je strani kljuc mozda je drugi tip
    pub storage_id: Option<String>,     //ovo je strani kljuc mozda je drugi tip
    pub slot: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "storage_slots"]
pub struct NewSTRGslots {
    pub motherboard_id: Option<String>, // ovo je strani kljuc mozda je drugi tip
    pub storage_id: Option<String>,     //ovo je strani kljuc mozda je drugi tip
    pub slot: String,
}
