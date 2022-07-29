use serde::{Serialize, Deserialize};

use crate::database::schema::build_size;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "build_size"]
pub struct BuildSize {
    pub motherboard_id: String,
    pub case_id: String,
    pub size: String,
}
