use serde::{Deserialize, Serialize};
use crate::database::motherboard::data::Motherboard;

#[derive(Deserialize)]
pub struct UserRequest{
    pub price: f32,
    pub priority: Option<String>,
}

#[derive(Serialize)]
pub struct GeneratedBuild{
    pub motherboard_name: String,
    pub cpu_name: String,
    pub gpu_name: String,
    pub storage_name: Vec<String>,
    pub ram_name: Vec<String>,
    pub pu_name: String,
    pub case_name: String,
    pub cooling: Vec<String>,
    pub price: f32,
}

#[derive(Deserialize)]
pub struct GetBuild{
    pub motherboard_name: String,
    pub cpu_name: String,
    pub gpu_name: String,
    pub storage_name: Vec<String>,
    pub ram_name: Vec<String>,
    pub pu_name: String,
    pub case_name: String,
    pub cooling: Vec<String>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct JoinedData{
    pub mb: Motherboard,
    pub cpu_name: String,
    pub stor_name: String,
    pub ram_name: String,
    pub case_name: String,
    pub mb_price: f32,
    pub cpu_price: f32,
    pub stor_price: f32,
    pub ram_price: f32,
    pub case_price: f32
}
