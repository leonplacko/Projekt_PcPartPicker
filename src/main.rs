#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::cookie::time::{Duration as cDuration};
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use std::env;


pub mod database;
pub mod generate_build;
pub mod db_web_setup;
pub mod response_error_handling;

use db_web_setup::*;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
  

    let conn_pool = establish_conn().await;
    let (domain, secret_key) = params().await;


    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret_key.as_bytes())
                .name("log_user")
                .path("/")
                .domain(domain.as_str())
                .max_age(cDuration::hours(2))
                .secure(false)
            ))
            .app_data(Data::new(conn_pool.clone()))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::scope("").configure(myconfig))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
