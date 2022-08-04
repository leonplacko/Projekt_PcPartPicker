use actix_identity::Identity;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use crate::response_error_handling::ErrorHandler;
use super::contract::CRUD;
use super::data::*;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

fn check_admin(id: &Identity) -> (bool, HttpResponse){
    let ident = match id.identity(){
        Some(val) => val,
        None => String::from("None"),
    };

    if ident == "None".to_string(){
        return (false, ErrorHandler::Unauthorized("No user login").get_response())
    }else{
        let admin: &str = ident.split(":").collect::<Vec<&str>>()[1];
        if admin == false.to_string(){
            return (false, ErrorHandler::Unauthorized("Unauthorized").get_response())
        }else{
            return (true, HttpResponse::Ok().json(""))
        } 
    }

}

pub async fn get_power_unit(conn: Data<DBPool>, id: Identity) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    match check{
        (false, er) => er,
        (true, _) => match PowerUnit::read_all(&pool){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(val),
            Err(er) => ErrorHandler::from(er).get_response(),
        }
    }
}

pub async fn insert_power_unit(conn: Data<DBPool>, id: Identity, data: Json<NewPowerUnit>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    match check{
        (false, er) => er,
        (true, _) => match PowerUnit::create(data.0, &pool){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("Power unit inserted with id: {}", val.id)),
            Err(er) => ErrorHandler::from(er).get_response(),
        }
    }
}

pub async fn delete_power_unit(conn: Data<DBPool>, id: Identity, data: Json<String>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    match check{
        (false, er) => er,
        (true, _) => match PowerUnit::delete(data.0, &pool){
            Ok(_) => HttpResponse::Ok().content_type("application/json").json("Powerunit succesfully deleted"),
            Err(er) => ErrorHandler::from(er).get_response(),
        }
    }
}

pub async fn update_power_unit(conn: Data<DBPool>, id: Identity, data: Json<NewPowerUnit>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    match check{
        (true, er) => er,
        (false, _) => match PowerUnit::update(&pool, data.0){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("Power unit with id: {} updated", val.id)),
            Err(er) => ErrorHandler::from(er).get_response(),
        }
    }
}