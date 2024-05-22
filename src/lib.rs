use worker::*;

use tower_service::Service;
mod helpers;
mod routes;

#[event(fetch)]
async fn fetch(
	req: HttpRequest,
	env: Env,
	_ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
	console_error_panic_hook::set_once();
	Ok(routes::router(env).call(req).await?)
}
