use actix_web::{
    body::BoxBody, get, http::header::ContentType, post, web, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// This struct represents an author
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl Responder for Author {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/author/{id}")]
pub async fn get_author(id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let authors = data.authors.lock().unwrap();
    for author in authors.authors {
        if author.id = id {
            return author;
        }
    }

    HttpResponse::NotFound()
        .body("No author found for id: {id}")
}

#[post("/author")]
pub async fn create_author(author: web::Json<Author>, data: web::Data<AppState>) -> impl Responder {
    let mut authors = data.authors.lock().unwrap();
    authors.authors.push(author.clone());

    author
}

// This struct represents a collection of Authors
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Authors {
    pub authors: Vec<Author>,
}

impl Responder for Authors {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/authors")]
pub async fn list_authors(data: web::Data<AppState>) -> impl Responder {
    let authors = data.authors.lock().unwrap().clone();
    println!("authors: {:?}", authors);

    authors
}

// This struct represents state
pub struct AppState {
    pub app_name: String,
    pub counter: Mutex<i32>,
    pub authors: Mutex<Authors>,
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Hello {name}! app_name={app_name}, request number={counter}")
}
