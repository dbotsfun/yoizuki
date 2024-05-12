use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
	/// Event name
	pub name: String,
	/// Event description
	pub description: String,
}

#[post("/event")]
pub async fn post_event(event: web::Form<Event>) -> HttpResponse {
	HttpResponse::Ok().json(event.into_inner())
}
