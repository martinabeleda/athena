use actix_web::{get, web, Responder};
use std::sync::Mutex;

// This struct represents state
pub struct AppState {
    pub app_name: String,
    pub counter: Mutex<i32>,
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Hello {name}! app_name={app_name}, request number={counter}")
}
