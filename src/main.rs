use actix_web::{middleware, App, HttpServer};
use std::env;
use url_shortener::{cache, routes, storage::Storage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port_str = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port_str
        .parse::<u16>()
        .expect("SERVER_PORT must be a number");
    log::info!("starting HTTP server at :8080");
    // connect to database
    let db = Storage::new();
    let redis: cache::Redis = cache::Redis::new();
    HttpServer::new(move || {
        App::new()
            // enable logger
            .configure(routes::configure)
            .app_data(db.clone())
            .app_data(redis.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
