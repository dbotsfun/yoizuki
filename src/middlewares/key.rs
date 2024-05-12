use futures_util::FutureExt;
use std::{
	future::{ready, Ready},
	rc::Rc,
};

use actix_web::{
	body::EitherBody,
	dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
	Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::helpers::responses::CustomResponse;

pub struct Key;

impl<S, B> Transform<S, ServiceRequest> for Key
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type InitError = ();
	type Transform = KeyMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		let service = Rc::new(service);
		ready(Ok(KeyMiddleware { service }))
	}
}

pub struct KeyMiddleware<S> {
	service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for KeyMiddleware<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	dev::forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let request = req.request().clone();
		let key = match request.headers().get("Authorization") {
			Some(key) => key.to_str().unwrap_or("Unknown"),
			None => "Unknown",
		};

		let secret_key = std::env::var("SECRET_KEY").unwrap_or("".to_string());

		if key.len() < 1 || key != secret_key {
			return Box::pin(async move {
				Ok(req.into_response(
					HttpResponse::Unauthorized()
						.json(CustomResponse {
							message: String::from(
								"Unauthorized access! Please provide a valid key.",
							),
						})
						.map_into_right_body(),
				))
			});
		}

		let service = Rc::clone(&self.service);

		async move {
			service
				.call(req)
				.await
				.map(ServiceResponse::map_into_left_body)
		}
		.boxed_local()
	}
}
