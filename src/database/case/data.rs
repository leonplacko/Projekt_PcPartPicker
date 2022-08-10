use serde::{Deserialize, Serialize};

use crate::database::schema::pc_case;

#[derive(Queryable, Debug, Serialize)]
pub struct Case {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub dimensions: String,
    pub ventilators: i32,
    pub available_vent: i32,
    pub price: f32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "pc_case"]
pub struct NewCase {
    pub name: String,
    pub manufacturer: String,
    pub dimensions: String,
    pub ventilators: i32,
    pub available_vent: i32,
    pub price: f32,
}

#[derive(Deserialize)]
pub struct ExtendCase {
    pub name: String,
    pub manufacturer: String,
    pub dimensions: String,
    pub ventilators: i32,
    pub available_vent: i32,
    pub price: f32,
    pub build_size: String,
}

impl From<ExtendCase> for NewCase {
    fn from(ex: ExtendCase) -> Self {
        NewCase {
            name: ex.name.to_owned(),
            manufacturer: ex.manufacturer.to_owned(),
            dimensions: ex.dimensions.to_owned(),
            ventilators: ex.ventilators.to_owned(),
            available_vent: ex.available_vent.to_owned(),
            price: ex.price.to_owned(),
        }
    }
}
