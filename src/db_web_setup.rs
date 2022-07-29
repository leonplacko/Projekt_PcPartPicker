use actix_web::web;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use dotenv::dotenv;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


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
    //stavi servise sve
}


