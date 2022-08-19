use actix_web::{get, post, web, HttpResponse};

use crate::authors::models::{Author, Authors};

#[get("/authors")]
pub async fn find_all() -> HttpResponse {
    let authors = Authors::find_all().expect("Couldn't fetch authors");
    HttpResponse::Ok().json(authors)
}

#[get("/authors/{id}")]
pub async fn find(id: web::Path<i32>) -> HttpResponse {
    let author = Authors::find(id.into_inner()).expect("No author for id: {id}");
    HttpResponse::Ok().json(author)
}

#[post("/authors")]
pub async fn create(author: web::Json<Author>) -> HttpResponse {
    let author = Authors::create(author.into_inner()).expect("Unable to create author");
    HttpResponse::Ok().json(author)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
}
