use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{id}").service(web::resource("").route(web::get().to(handlers::models::get))),
    );
    cfg.service(
        web::scope("/")
            .service(web::resource("").route(web::post().to(handlers::models::create)))
            .wrap(HttpAuthentication::basic(handlers::models::basic_auth))
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        // create custom error response
                        actix_web::error::InternalError::from_response(
                            err,
                            actix_web::HttpResponse::BadRequest().finish(),
                        )
                        .into()
                    }),
            ),
    );
}
