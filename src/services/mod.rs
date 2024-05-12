use crate::helpers::responses::CustomResponse;
use actix_web::{get, web, HttpResponse};
mod event;

/// Main route
#[get("/")]
pub async fn main_route() -> HttpResponse {
    HttpResponse::Ok().json(CustomResponse {
        message: String::from("Hello, World!"),
    })
}

/// Initialize routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(event::post_event);
}
