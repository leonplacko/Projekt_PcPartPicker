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

pub async fn get_cooling(conn: Data<DBPool>, id: Identity) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");
    
    let check = check_admin(&id);

    if !check.0{
        return check.1
    }else{
        let result = Cooling::read_all(&pool);
        return match result{
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(val),
            Err(er) => ErrorHandler::from(er).get_response(),
        }
    }
        
}

pub async fn insert_cooling(conn: Data<DBPool>, id: Identity, data: Json<NewCooling>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);
    
    if !check.0{
        return check.1
    }else{
        let result = Cooling::create(data.0,&pool);
        return match result{
                Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("Cooling succesfully inserted with id: {}", val.id)),
                Err(er) => ErrorHandler::from(er).get_response(),
            }
    }
}

pub async fn update_cooling(conn: Data<DBPool>, id: Identity, data: Json<NewCooling>) -> HttpResponse{
    let pool = conn.get().expect("Error connecting to pool");

    let check = check_admin(&id);

    if !check.0{
        return check.1
    }else{
        let result = Cooling::update(&pool, data.0);
        return match result{
            Ok(_) => HttpResponse::Ok().content_type("application/json").json("Cooling succesfully updated"),
            Err(er) => ErrorHandler::from(er).get_response(),
        }
    }
}

pub async fn delete_cooling(conn: Data<DBPool>, id: Identity, data: Json<String>) -> HttpResponse{
    let pool = conn.get().expect("Error connection to pool");

    let check = check_admin(&id);

    if !check.0{
        return check.1
    }else{
        let result = Cooling::delete(data.0, &pool);
        return match result{
                Ok(_) => HttpResponse::Ok().content_type("application/json").json("Cooling deleted succesfully"),
                Err(er) => ErrorHandler::from(er).get_response(),
            }
    }
}