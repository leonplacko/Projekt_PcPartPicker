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


pub async fn get_cpu(conn: Data<DBPool>, id: Identity) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    
    match check_admin(&id){
        (false, er) => er,
        (true, _) => match CPU::read_all(&pool){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(val),
            Err(er) => ErrorHandler::from(er).get_response(),
        } 
    }
}

pub async fn insert_cpu(conn: Data<DBPool>, id: Identity, data: Json<ExtendCPU>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    println!("prošao");

    match check_admin(&id){
        (false, er) => er,
        (true, _) => match CPU::create(data.0, &pool){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("CPU inserted with id {}", val.id)),
            Err(er) => ErrorHandler::from(er).get_response(),
        } 
    }


}

pub async fn delete_cpu(conn: Data<DBPool>, id: Identity, data: Json<String>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    match check_admin(&id){
        (false, er) => er,
        (true, _) => match CPU::delete(data.0, &pool){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("CPU succesfully deleted, amount: {}", val)),
            Err(er) => ErrorHandler::from(er).get_response(),
        } 
    }
}

pub async fn update_cpu(conn: Data<DBPool>, id: Identity, data: Json<NewCPU>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    match check_admin(&id){
        (false, er) => er,
        (true, _) => match CPU::update(&pool, data.0){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("CPU updated with id: {}", val.id)),
            Err(er) => ErrorHandler::from(er).get_response(),
        } 
    }
}