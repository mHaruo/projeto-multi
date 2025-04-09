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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
    
        App::new()
            .wrap(cors)
            .configure(config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    
}
