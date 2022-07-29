use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserRequest{
    price: f32,
    priority: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TheBuild{
    motherboard_name: String,
    cpu_name: String,
    gpu_name: String,
    storage_name: Vec<String>,
    ram_name: Vec<String>,
    pu_name: String,
    case_name: String,
    cooling: Vec<String>,
}
