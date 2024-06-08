pub mod event;

use crate::{helpers::responses::CustomResponse, middlewares::auth::auth_middleware, AppState};
use axum::{
	middleware,
	routing::{get, post},
	Extension, Json, Router,
};
use worker::Env;

pub async fn main_route() -> Json<CustomResponse> {
	Json(CustomResponse {
		message: "Hello, World!".to_string(),
	})
}

pub fn router(env: Env) -> Router {
	let state = AppState { env: env.clone() };

	Router::new()
		.route("/", get(main_route))
		.route("/event", post(event::post_event))
		.layer(middleware::from_fn_with_state(
			state.clone(),
			auth_middleware,
		))
		.with_state(state)
		.layer(Extension(env))
}
