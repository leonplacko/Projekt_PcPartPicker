use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

use actix_web::{post, web::{self, Data, Json}, HttpResponse};

use super::build_models::*;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[post("user/{username}/create_build")]
pub async fn gen_price(path: web::Path<(String)>, conn: Data<DBPool>, data: Json<UserRequest>) -> HttpResponse{
    let username = path.into_inner();
    
    
    HttpResponse::Ok().content_type("application/json").json("Ok")
}




