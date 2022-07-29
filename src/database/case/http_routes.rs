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

pub async fn get_all(conn: Data<DBPool>, id: Identity) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");
    
    let ident = match id.identity(){
        Some(val) => val,
        None => String::from("None"),
    };

    if ident == "None".to_string(){
        return ErrorHandler::Unauthorized("No user login").get_response()
    }else{
        let admin: &str = ident.split(":").collect::<Vec<&str>>()[1];
        if admin == false.to_string(){
            return ErrorHandler::Unauthorized("Unauthorized").get_response()
        }else{
            let result = Case::read_all(&pool);
            match result{
                Ok(val) => HttpResponse::Ok().content_type("application/json").json(val),
                Err(er) => ErrorHandler::from(er).get_response(),
            }
        }
    }

}

pub async fn insert(conn: Data<DBPool>, id: Identity, data: Json<NewCase>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");
    
    let ident = match id.identity(){
        Some(val) => val,
        None => String::from("None"),
    };

    if ident == "None".to_string(){
        return ErrorHandler::Unauthorized("No user login").get_response()
    }else{
        let admin: &str = ident.split(":").collect::<Vec<&str>>()[1];
        if admin == false.to_string(){
            return ErrorHandler::Unauthorized("Unauthorized").get_response()
        }else{
            let result = Case::create(data.0,&pool);
            match result{
                Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("Case succesfully inserted with id: {}", val.id)),
                Err(er) => ErrorHandler::from(er).get_response(),
            }
        }
    }
}

pub async fn delete(conn: Data<DBPool>, id: Identity, data: Json<String>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");
    
    let ident = match id.identity(){
        Some(val) => val,
        None => String::from("None"),
    };

    if ident == "None".to_string(){
        return ErrorHandler::Unauthorized("No user login").get_response()
    }else{
        let admin: &str = ident.split(":").collect::<Vec<&str>>()[1];
        if admin == false.to_string(){
            return ErrorHandler::Unauthorized("Unauthorized").get_response()
        }else{
            let result = Case::delete(data.0, &pool);
            match result{
                Ok(_) => HttpResponse::Ok().content_type("application/json").json("Case deleted succesfully"),
                Err(er) => ErrorHandler::from(er).get_response(),
            }
        }
    }
}

pub async fn update(conn: Data<DBPool>, id: Identity, data: Json<Case>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");
    
    let ident = match id.identity(){
        Some(val) => val,
        None => String::from("None"),
    };

    if ident == "None".to_string(){
        return ErrorHandler::Unauthorized("No user login").get_response()
    }else{
        let admin: &str = ident.split(":").collect::<Vec<&str>>()[1];
        if admin == false.to_string(){
            return ErrorHandler::Unauthorized("Unauthorized").get_response()
        }else{
            let result = Case::update(&pool, data.0);
            match result{
                Ok(_) => HttpResponse::Ok().content_type("application/json").json("Case succesfully updated"),
                Err(er) => ErrorHandler::from(er).get_response(),
            }
        }
    }
}


