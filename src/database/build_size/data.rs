use serde::{Deserialize, Serialize};

use crate::database::schema::build_size;

#[derive(Queryable, Debug, Serialize)]
pub struct BuildSize {
    pub id: String,
    pub motherboard_id: Option<String>,
    pub case_id: Option<String>,
    pub size: String,
}
#[derive(Insertable, Deserialize)]
#[table_name = "build_size"]
pub struct NewBuildSize {
    pub motherboard_id: Option<String>,
    pub case_id: Option<String>,
    pub size: String,
}
