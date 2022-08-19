#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};

mod authors;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let host = "0.0.0.0";
    println!(
        "Starting athena service on host: {:?} port: {:?}",
        host, port
    );

    HttpServer::new(move || {
        App::new().service(web::scope("/athena/v1").configure(authors::init_routes))
    })
    .bind((host, port))?
    .run()
    .await
}
