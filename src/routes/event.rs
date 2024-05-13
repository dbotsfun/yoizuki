use axum::{extract, Json};
use serde::{Deserialize, Serialize};

use crate::helpers::responses::CustomResponse;

#[derive(Deserialize)]
pub struct Payload {
	pub user_id: String,
	pub bot_id: String,
	pub query: String,
	pub name: String,
	pub webhook_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPayload {
	pub user_id: String,
	pub bot_id: String,
	pub query: String,
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	/// The name of the event
	pub name: String,
	pub payload: ReturnPayload,
}

pub async fn post_event(
	extract::Json(payload): extract::Json<Payload>,
) -> Result<Json<CustomResponse>, Json<CustomResponse>> {
	let return_payload = ReturnPayload {
		user_id: payload.user_id.clone(),
		bot_id: payload.bot_id.clone(),
		query: payload.query.clone(),
		name: payload.name.clone(),
	};

	let response = reqwest::Client::new()
		.post(&payload.webhook_url)
		.json(&Event {
			name: payload.name.clone(),
			payload: return_payload,
		})
		.send()
		.await;

	match response {
		Ok(response) => {
			if response.status().is_success() {
				Ok(Json(CustomResponse {
					message: "Event sent successfully".to_string(),
				}))
			} else {
				Ok(Json(CustomResponse {
					message: format!("Failed to send event: {}", response.status()),
				}))
			}
		}
		Err(e) => Ok(Json(CustomResponse {
			message: format!("Failed to send event: {}", e),
		})),
	}
}
