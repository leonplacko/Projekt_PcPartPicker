use actix_identity::Identity;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::response_error_handling::ErrorHandler;
use super::contract::CRUD;
use super::data::*;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;


fn checkin(val: User, hashed_pass: String, id: Identity) -> HttpResponse{
    if let Some(_) = id.identity(){
        return ErrorHandler::Unauthorized("User already logged in").get_response()
    }else if val.password == hashed_pass{
        let user_remember = format!("{}:{}", val.username.clone(), val.isadmin.to_string());
        //let a = id.identity();
        id.remember(user_remember.clone());
        return HttpResponse::Ok().content_type("application/json").json(format!("Succesful login with identity: {}", user_remember.clone()))
    }else{
        return ErrorHandler::BadRequest("Wrong credentials").get_response()
    }
}

//#[get("/users")]
pub async fn get_users(conn: Data<DBPool>, id: Identity) -> HttpResponse {
    let pool = conn.get().expect("Error with connection pool");
    let ident = match id.identity(){
        Some(val) => val,
        None => "None".to_string(),
    };
    

    let values: Vec<&str> = ident.split(":").collect();
    
    if ident == "None"{
        return ErrorHandler::NotFound("User is not logged in").get_response()
        
    }else if values[1] == true.to_string(){
        match User::read_all(&pool){
            Ok(val) => HttpResponse::Ok().content_type("application/json").json(val),
            Err(er) => ErrorHandler::from(er).get_response(),
        }
    }else{
        return ErrorHandler::Unauthorized("Unauthorized").get_response()
    }

    
}

pub async fn register(conn: Data<DBPool>, data: Json<NewUser>) -> HttpResponse{
    let pool = conn.get().expect("Error with connection pool");
    let myuser = User::read_user(data.0.username.clone(), &pool);

    let result = match myuser{
        Ok(val) => val.username,
        Err(diesel::result::Error::NotFound) => "".to_string(),
        Err(er) => er.to_string()
    };

    
    if result == data.0.username.clone(){
        return ErrorHandler::BadRequest("User already exists").get_response()
    }else if result.len() != 0{
        return ErrorHandler::InternalServerError(format!("Error with diesel query: {}", result)).get_response()
    }else{
        
        let user = data.0;
        if user.password.clone().trim().len() == 0{
            return ErrorHandler::BadRequest("Empty password").get_response()
        }else{

            return match User::create(user, &pool){
                Ok(val) => HttpResponse::Ok().content_type("application/json").json(format!("Inserted user with id: {}", val.id)),
                Err(er) => ErrorHandler::from(er).get_response(),
            }

        }
        
         
    }
}

pub async fn login(conn: Data<DBPool>, data: Json<UserLog>, id: Identity) -> HttpResponse{
    let pool = conn.get().expect("Error with connection pool");
    let myuser = User::read_user(data.0.username.clone(), &pool);

    let mut hasher = DefaultHasher::new();
    data.0.password.clone().hash(&mut hasher);
    let hashed_pass = format!("{:x}", hasher.finish());
    

    match myuser{
        Ok(val) => checkin(val, hashed_pass, id),    
        Err(er) => ErrorHandler::from(er).get_response(),
    }

}

pub async fn logout(id: Identity) -> HttpResponse{
    id.forget();
    HttpResponse::Ok().content_type("application/json").json("Logout succesfull")
}
