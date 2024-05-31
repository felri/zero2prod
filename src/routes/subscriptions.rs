use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(_form: web::Json<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
