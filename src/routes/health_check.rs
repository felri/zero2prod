use actix_web::{HttpResponse, Responder};

#[tracing::instrument(name = "Checking service health", skip())]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
