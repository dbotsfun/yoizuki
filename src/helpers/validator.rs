use axum::{
	async_trait,
	extract::{rejection::FormRejection, Form, FromRequest, Request},
	response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
	T: DeserializeOwned + Validate,
	S: Send + Sync,
	Form<T>: FromRequest<S, Rejection = FormRejection>,
{
	type Rejection = ServerError;

	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
		let Form(value) = Form::<T>::from_request(req, state).await?;
		value.validate()?;
		Ok(ValidatedForm(value))
	}
}

#[derive(Debug, Error)]
pub enum ServerError {
	#[error(transparent)]
	ValidationError(#[from] validator::ValidationErrors),

	#[error(transparent)]
	AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
	fn into_response(self) -> Response {
		match self {
			ServerError::ValidationError(_) => {
				let message = format!("Input validation error: [{self}]").replace('\n', ", ");
				(StatusCode::BAD_REQUEST, message)
			}
			ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
		}
		.into_response()
	}
}
