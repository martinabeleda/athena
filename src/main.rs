use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use athena::{create_author, get_author, greet, list_authors, AppState, Author, Authors};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        app_name: String::from("Some Data"),
        authors: Mutex::new(Authors {
            authors: vec![Author {
                id: 0,
                first_name: String::from("Haruki"),
                last_name: String::from("Murakami"),
            }],
        }),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new().app_data(counter.clone()).service(
            web::scope("/athena/v1")
                .service(create_author)
                .service(greet)
                .service(get_author)
                .service(list_authors),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
