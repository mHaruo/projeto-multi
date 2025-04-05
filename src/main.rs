mod models;
mod handlers;
mod storage;

use handlers::config;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // 👈 Habilita CORS para tudo (temporariamente)
            .configure(config) // 👈 Correção aqui
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
