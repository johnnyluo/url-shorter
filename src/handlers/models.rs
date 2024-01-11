use actix_web::dev::ServiceRequest;
use actix_web::{error::ErrorUnauthorized, http::header, web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub user_id: String,
    pub target: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlShortenResult {
    pub id: String,
}

/// create a new shortened url
pub async fn create(payload: web::Json<Payload>) -> impl Responder {
    log::debug!("payload: {:?}", payload);
    log::debug!("user_id: {} , target: {} ", payload.user_id, payload.target);
    HttpResponse::Ok().json(payload.0)
}

pub async fn get(id: web::Path<String>) -> impl Responder {
    log::debug!("url id: {}", id);
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "https://www.google.com"))
        .finish()
}

/// implement basic auth
pub async fn basic_auth(
    req: ServiceRequest,
    credentials: actix_web_httpauth::extractors::basic::BasicAuth,
) -> Result<ServiceRequest, Error> {
    if credentials.user_id() == "admin"
        && credentials.password().map(|s| s.as_ref()) == Some("password")
    {
        Ok(req)
    } else {
        Err(ErrorUnauthorized("Invalid Credential"))
    }
}
