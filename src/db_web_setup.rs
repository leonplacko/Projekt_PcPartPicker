use crate::database::{
    CPU::http_routes as CpuHttp,
    GPU::http_routes as GpuHttp,
    case::http_routes as CaseHttp,
    cooling::http_routes as CoolingHttp,
    motherboard::http_routes as MbHttp,
    power_unit::http_routes as PowerUnitHttp,
    RAM::http_routes as RamHttp,
    storage::http_routes as StorageHttp,
    users::http_routes as UserHttp,
};

use crate::generate_build::generate_build::*;

use actix_web::{web, get, HttpResponse};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use dotenv::dotenv;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


#[get("/")]
async fn intro() -> HttpResponse{
    HttpResponse::Ok().body("Welcome to PcPartPicker!")
}

pub async fn params() -> (String, String) {
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let mut hasher = DefaultHasher::new();
    std::env::var("SECRET_KEY").unwrap().hash(&mut hasher);
    let secret_key = format!("{:x}", hasher.finish()) + &format!("{:x}", hasher.finish());

    (domain, secret_key)
}

pub async fn establish_conn() -> DBPool{
    dotenv().ok();
    
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not detected.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).expect("Failed to initialize DB connection pool.")
}

pub fn myconfig(cfg: &mut web::ServiceConfig){
    //GET
    cfg.service(web::resource("cpus").route(web::get().to(CpuHttp::get_cpu)));
    cfg.service(web::resource("gpus").route(web::get().to(GpuHttp::get_gpu)));
    cfg.service(web::resource("storages").route(web::get().to(StorageHttp::get_storage)));
    cfg.service(web::resource("cases").route(web::get().to(CaseHttp::get_case)));
    cfg.service(web::resource("coolings").route(web::get().to(CoolingHttp::get_cooling)));
    cfg.service(web::resource("motherboards").route(web::get().to(MbHttp::get_motherboard)));
    cfg.service(web::resource("power_units").route(web::get().to(PowerUnitHttp::get_power_unit)));
    cfg.service(web::resource("rams").route(web::get().to(RamHttp::get_ram)));

    //POST
    cfg.service(web::resource("cpu/insert").route(web::post().to(CpuHttp::insert_cpu)));
    cfg.service(web::resource("gpu/insert").route(web::post().to(GpuHttp::insert_gpu)));
    cfg.service(web::resource("storage/insert").route(web::post().to(StorageHttp::insert_storage)));
    cfg.service(web::resource("case/insert").route(web::post().to(CaseHttp::insert_case)));
    cfg.service(web::resource("cooling/insert").route(web::post().to(CoolingHttp::insert_cooling)));
    cfg.service(web::resource("motherboard/insert").route(web::post().to(MbHttp::insert_motherboard)));
    cfg.service(web::resource("power_unit/insert").route(web::post().to(PowerUnitHttp::insert_power_unit)));
    cfg.service(web::resource("ram/insert").route(web::post().to(RamHttp::insert_ram)));

    //PUT
    cfg.service(web::resource("cpu/update").route(web::put().to(CpuHttp::update_cpu)));
    cfg.service(web::resource("gpu/update").route(web::put().to(GpuHttp::update_gpu)));
    cfg.service(web::resource("storage/update").route(web::put().to(StorageHttp::update_storage)));
    cfg.service(web::resource("case/update").route(web::put().to(CaseHttp::update_case)));
    cfg.service(web::resource("cooling/update").route(web::put().to(CoolingHttp::update_cooling)));
    cfg.service(web::resource("motherboard/update").route(web::put().to(MbHttp::update_motherboard)));
    cfg.service(web::resource("power_unit/update").route(web::put().to(PowerUnitHttp::update_power_unit)));
    cfg.service(web::resource("ram/update").route(web::put().to(RamHttp::update_ram)));

    //DELETE
    cfg.service(web::resource("cpu/delete").route(web::delete().to(CpuHttp::delete_cpu)));
    cfg.service(web::resource("gpu/delete").route(web::delete().to(GpuHttp::delete_gpu)));
    cfg.service(web::resource("storage/delete").route(web::delete().to(StorageHttp::delete_storage)));
    cfg.service(web::resource("case/delete").route(web::delete().to(CaseHttp::delete_case)));
    cfg.service(web::resource("cooling/delete").route(web::delete().to(CoolingHttp::delete_cooling)));
    cfg.service(web::resource("motherboard/delete").route(web::delete().to(MbHttp::delete_motherboard)));
    cfg.service(web::resource("power_unit/delete").route(web::delete().to(PowerUnitHttp::delete_power_unit)));
    cfg.service(web::resource("ram/delete").route(web::delete().to(RamHttp::delete_ram)));

    //GENERATOR
    cfg.service(web::resource("user/{username}/create_build").route(web::post().to(gen_price)));
    //cfg.service(web::resource("user/{username}/validate_build").route(web::post().to(val_build)));

    //USER
    cfg.service(web::resource("user").route(web::get().to(UserHttp::get_users)));    
    cfg.service(web::resource("user/register").route(web::post().to(UserHttp::register)));
    cfg.service(web::resource("user/login").route(web::post().to(UserHttp::login)));
    cfg.service(web::resource("user/logout").route(web::post().to(UserHttp::logout)));
    cfg.service(intro);
}


