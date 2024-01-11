use actix_web::{middleware, App, HttpServer};
use url_shortener::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at :8080");
    HttpServer::new(|| {
        App::new()
            // enable logger
            .configure(routes::configure)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
