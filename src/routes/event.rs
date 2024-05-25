use axum::Json;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::helpers::{responses::CustomResponse, validator::ValidatedForm};

#[derive(Deserialize, Validate)]
pub struct Payload {
	#[validate(length(min = 18, max = 19, message = "Invalid user ID"))]
	#[serde(rename = "userId")]
	pub user_id: String,

	#[validate(length(min = 18, max = 19, message = "Invalid bot ID"))]
	#[serde(rename = "botId")]
	pub bot_id: String,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

	pub secret: String,

	#[validate(url)]
	#[serde(rename = "webhookUrl")]
	pub webhook_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPayload {
	pub user_id: String,
	pub bot_id: String,
	pub secret: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	/// The name of the event
	pub name: String,
	pub payload: ReturnPayload,
}

/// Response type for the event endpoint
pub type EventResponse = (StatusCode, Json<CustomResponse>);

#[worker::send]
pub async fn post_event(
	ValidatedForm(payload): ValidatedForm<Payload>,
) -> Result<EventResponse, EventResponse> {
	let return_payload = ReturnPayload {
        name: payload.name.clone(),
		user_id: payload.user_id.clone(),
		bot_id: payload.bot_id.clone(),
		secret: payload.secret.clone(),
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
				Ok((
					StatusCode::OK,
					Json(CustomResponse {
						message: "Event sent successfully".to_string(),
					}),
				))
			} else {
				Ok((
					StatusCode::OK,
					Json(CustomResponse {
						message: format!("Failed to send event: {}", response.status()),
					}),
				))
			}
		}
		Err(e) => Err((
			StatusCode::INTERNAL_SERVER_ERROR,
			Json(CustomResponse {
				message: format!("Failed to send event: {}", e),
			}),
		)),
	}
}
