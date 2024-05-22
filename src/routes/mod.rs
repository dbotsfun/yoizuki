use crate::helpers::responses::CustomResponse;
use axum::{
	routing::{get, post},
	Extension, Json, Router,
};
use worker::Env;

pub mod event;

pub async fn main_route() -> Json<CustomResponse> {
	Json(CustomResponse {
		message: "Hello, World!".to_string(),
	})
}

pub fn router(env: Env) -> Router {
	Router::new()
		.route("/", get(main_route))
		.route("/event", post(event::post_event))
		.layer(Extension(env))
}
