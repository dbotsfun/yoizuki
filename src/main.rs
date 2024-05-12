use actix_cors::Cors;
use actix_web::{http::header, middleware};
use services::{init_routes, main_route};
mod helpers;
mod middlewares;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

	let port = std::env::var("PORT")
		.unwrap_or("8080".to_string())
		.parse::<u16>()
		.unwrap();

	let server = actix_web::HttpServer::new(|| {
		actix_web::App::new()
			.app_data(actix_web::web::JsonConfig::default().limit(4096)) // Limit size of payload
			.wrap(middleware::Compress::default())
			.wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
			.wrap(middlewares::key::Key)
			.wrap(
				Cors::default()
					.allow_any_origin()
					.allowed_methods(vec!["GET", "POST"])
					.allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE]),
			)
			.service(main_route)
			.configure(init_routes)
	});

	log::info!("Server running on port: {}", port);

	let _ = server.bind(("0.0.0.0", port))?.run().await;

	Ok(())
}
