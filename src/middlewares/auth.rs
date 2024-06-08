use crate::AppState;
use axum::{
	extract::{Request, State},
	middleware::Next,
	response::Response,
};
use reqwest::StatusCode;

pub async fn auth_middleware(
	State(state): State<AppState>,
	req: Request,
	next: Next,
) -> Result<Response, StatusCode> {
	let auth_header = req.headers().get("Authorization");
	let key = state
		.env
		.var("API_KEY")
		.expect("API_KEY not found")
		.to_string();

	if let Some(auth_header) = auth_header {
		let auth_header = auth_header.to_str().unwrap();
		if auth_header.eq(&format!("Bearer {}", key)) {
			return Ok(next.run(req).await);
		}
	}

	Err(StatusCode::UNAUTHORIZED)
}
