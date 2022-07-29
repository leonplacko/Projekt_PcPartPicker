use serde::{Serialize, Deserialize};

use crate::database::schema::socket;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "socket"]
pub struct Socket {
    pub motherboard_id: String, // ovo je strani kljuc mozda je drugi tip
    pub cpu_id: String,         // ovo je strani kljuc mozda je drugi tip
    pub socket_type: String,
}
