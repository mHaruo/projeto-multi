mod models;
mod user;
mod storage;
mod project;

use user::config;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) 
            .configure(config) 
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
