use actix_web::{App, HttpServer};

mod controllers;
mod models;
mod services;

use controllers::get_all_routers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_routers()))
        .bind("127.0.0.1:4000")?
        .run()
        .await
}
