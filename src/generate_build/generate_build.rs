use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

use actix_web::{web::{Path, Data, Json}, HttpResponse};

use super::build_models::*;

use crate::database::{
    CPU::infrastructure as Cpu_inf,
    GPU::infrastructure as Gpu_inf,
    RAM::infrastructure as Ram_inf,
    storage::infrastructure as Stor_inf,
    power_unit::infrasctructure as Power_inf,
    motherboard::infrasctructure as MB_inf,
    cooling::infrastructure as Cool_inf,
    case::infrastructure as Case_inf,
};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

//#[post("user/{username}/create_build")]
pub async fn gen_price(path: Path<String>, conn: Data<DBPool>, data: Json<UserRequest>) -> HttpResponse{
    let username = path.into_inner();
    
    let (price, prio) = match data.0.priority{
        Some(val) => (data.0.price, val),
        None => (data.0.price, String::from("")),
    };
    
    
    
    HttpResponse::Ok().content_type("application/json").json("Ok")
}

pub async fn val_build(path: Path<String>, conn: Data<DBPool>, data: Json<GetBuild>) -> HttpResponse{
    HttpResponse::Ok().content_type("application/json").json("")
}




