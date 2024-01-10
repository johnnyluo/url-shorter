use actix_web::web;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{id}")
            .service(web::resource("").route(web::get().to(|| async { "create user" }))),
    );
    cfg.service(
        web::scope("/").service(web::resource("").route(web::post().to(|| async { "get users" }))),
    );
}
