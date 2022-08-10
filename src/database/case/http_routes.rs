use actix_identity::Identity;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use super::contract::CRUD as caseCRUD;
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

pub async fn get_case(conn: Data<DBPool>, id: Identity) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    if !check.0 {
        return check.1;
    } else {
        let result = Case::read_all(&pool);
        return match result {
            Ok(val) => HttpResponse::Ok()
                .content_type("application/json")
                .json(val),
            Err(er) => ErrorHandler::from(er).get_response(),
        };
    }
}

pub async fn insert_case(conn: Data<DBPool>, id: Identity, data: Json<ExtendCase>) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    if !check.0 {
        return check.1;
    } else {
        let result = Case::create(data.0, &pool);
        return match result {
            Ok(val) => HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("Case succesfully inserted with id: {}", val.id)),
            Err(er) => ErrorHandler::from(er).get_response(),
        };
    }
}

pub async fn delete_case(conn: Data<DBPool>, id: Identity, data: Json<String>) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    if !check.0 {
        return check.1;
    } else {
        let result = Case::delete(data.0, &pool);
        return match result {
            Ok(val) => HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("Case deleted succesfully, amount: {}", val)),
            Err(er) => ErrorHandler::from(er).get_response(),
        };
    }
}

pub async fn update_case(conn: Data<DBPool>, id: Identity, data: Json<NewCase>) -> HttpResponse {
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    if !check.0 {
        return check.1;
    } else {
        let result = Case::update(&pool, data.0);
        return match result {
            Ok(_) => HttpResponse::Ok()
                .content_type("application/json")
                .json("Case succesfully updated"),
            Err(er) => ErrorHandler::from(er).get_response(),
        };
    }
}
