use actix_identity::Identity;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use super::contract::CRUD as storageCRUD;
use super::data::*;
use crate::response_error_handling::ErrorHandler;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

fn check_admin(id: &Identity) -> (bool, HttpResponse) {
    let ident = match id.identity() {
        Some(val) => val,
        None => String::from("None"),
    };

    if ident == "None".to_string() {
        return (
            false,
            ErrorHandler::Unauthorized("No user login").get_response(),
        );
    } else {
        let admin: &str = ident.split(":").collect::<Vec<&str>>()[1];
        if admin == false.to_string() {
            return (
                false,
                ErrorHandler::Unauthorized("Unauthorized").get_response(),
            );
        } else {
            return (true, HttpResponse::Ok().json(""));
        }
    }
}

pub async fn get_storage(conn: Data<DBPool>, id: Identity) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    match check_admin(&id) {
        (false, er) => er,
        (true, _) => match Storage::read_all(&pool) {
            Ok(val) => HttpResponse::Ok()
                .content_type("application/json")
                .json(val),
            Err(er) => ErrorHandler::from(er).get_response(),
        },
    }
}

pub async fn insert_storage(
    conn: Data<DBPool>,
    id: Identity,
    data: Json<ExtendStorage>,
) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    match check_admin(&id) {
        (false, er) => er,
        (true, _) => match Storage::create(data.0, &pool) {
            Ok(val) => HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("Storage inserted with id: {}", val.id)),
            Err(er) => ErrorHandler::from(er).get_response(),
        },
    }
}

pub async fn delete_storage(conn: Data<DBPool>, id: Identity, data: Json<String>) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    match check_admin(&id) {
        (false, er) => er,
        (true, _) => match Storage::delete(data.0, &pool) {
            Ok(val) => HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("Storage deleted, amount: {}", val)),
            Err(er) => ErrorHandler::from(er).get_response(),
        },
    }
}

pub async fn update_storage(
    conn: Data<DBPool>,
    id: Identity,
    data: Json<NewStorage>,
) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    match check_admin(&id) {
        (false, er) => er,
        (true, _) => match Storage::update(&pool, data.0) {
            Ok(val) => HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("Strorage updated with id: {}", val.id)),
            Err(er) => ErrorHandler::from(er).get_response(),
        },
    }
}
