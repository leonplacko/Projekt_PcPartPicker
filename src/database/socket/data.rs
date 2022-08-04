use serde::{Serialize, Deserialize};

use crate::database::schema::socket;

#[derive(Queryable, Debug, Serialize)]
pub struct Socket {
    pub id: String,
    pub motherboard_id: Option<String>, // ovo je strani kljuc mozda je drugi tip
    pub cpu_id: Option<String>,         // ovo je strani kljuc mozda je drugi tip
    pub socket_type: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "socket"]
pub struct NewSocket {
    pub motherboard_id: Option<String>, // ovo je strani kljuc mozda je drugi tip
    pub cpu_id: Option<String>,         // ovo je strani kljuc mozda je drugi tip
    pub socket_type: String,
}
