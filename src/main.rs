use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use athena::{get_author, greet, list_authors, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        app_name: String::from("Some Data"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new().app_data(counter.clone()).service(
            web::scope("/athena/v1")
                .service(greet)
                .service(get_author)
                .service(list_authors),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
