use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorInternalServerError;
use actix_web::{error::ErrorUnauthorized, http::header, web, Error, HttpResponse, Responder};
use actix_web::{HttpMessage, HttpRequest};
use md5;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub user_id: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlShortenResult {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: u64,
    pub username: String,
}

fn generate_shorten_id(payload: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let hash_digest = hasher.finalize();
    hex::encode(md5::compute(hash_digest).as_ref())
}
/// create a new shortened url
pub async fn create(payload: web::Json<Payload>, req: HttpRequest) -> impl Responder {
    if let Some(authenticated_user) = req.extensions().get::<AuthenticatedUser>() {
        if let Some(db) = req.app_data::<crate::storage::Storage>() {
            let payload_json = serde_json::to_string(&payload);
            if payload_json.is_err() {
                log::error!(
                    "fail to get the json payload, error: {}",
                    payload_json.err().unwrap()
                );
                return HttpResponse::InternalServerError().finish();
            }

            let shortened_id = generate_shorten_id(&payload_json.unwrap());
            let url_exist = db.get_url_by_shorten_id(&shortened_id);
            if url_exist.is_err() {
                log::error!(
                    "fail to check if url exist, error: {}",
                    url_exist.err().unwrap()
                );
                return HttpResponse::InternalServerError().finish();
            }

            if url_exist.unwrap().is_some() {
                log::debug!("url already exist");
                return HttpResponse::BadRequest().finish();
            }

            let url_id = db.save_url(authenticated_user.user_id, &payload.target, &shortened_id);
            if url_id.is_err() {
                log::error!("fail to save url,error:{}", url_id.unwrap_err());
                HttpResponse::InternalServerError().finish();
            } else {
                let result = UrlShortenResult { id: shortened_id };
                return HttpResponse::Ok().json(result);
            }
        } else {
            log::error!("no long term storage instance found");
            return HttpResponse::InternalServerError().finish();
        }
    } else {
        log::debug!("authenticated_user: None");
        return HttpResponse::Unauthorized().finish();
    }

    HttpResponse::InternalServerError().finish()
}

pub async fn get(id: web::Path<String>,req: HttpRequest) -> impl Responder {
    log::debug!("url id: {}", id);
    if let Some(db) = req.app_data::<crate::storage::Storage>() {
        let url = db.get_url_by_shorten_id(&id.trim().to_string());
        if url.is_err() {
            log::error!("fail to get url, error: {}", url.err().unwrap());
            return HttpResponse::InternalServerError().finish();
        }
        if let Some(url) = url.unwrap() {
            log::debug!("url: {}", url);
            return HttpResponse::TemporaryRedirect()
                .append_header((header::LOCATION, url))
                .finish();
        } else {
            HttpResponse::NotFound().finish()
        }
    } else {
        log::error!("no long term storage instance found");
        return HttpResponse::InternalServerError().finish();
    }
}

/// implement basic auth
pub async fn basic_auth(
    req: ServiceRequest,
    credentials: actix_web_httpauth::extractors::basic::BasicAuth,
) -> Result<ServiceRequest, Error> {
    let db = req.app_data::<crate::storage::Storage>();
    if db.is_none() {
        return Err(ErrorUnauthorized("No database found"));
    }
    let username = credentials.user_id().to_string();
    let password = credentials.password().unwrap().to_string();
    log::debug!("username: {}, password: {:?}", username, password);
    let user_result = db.unwrap().get_user(&username, &password);
    match user_result {
        Ok(user_id) => match user_id {
            Some(user_id) => {
                log::info!("user {username} ,found with id: {user_id}");
                let _ = req.request().extensions_mut().insert(AuthenticatedUser {
                    user_id,
                    username: username.clone(),
                });
                Ok(req)
            }
            None => {
                log::debug!("user {username} , with password :{password} not found");
                return Err(ErrorUnauthorized("Invalid Credential"));
            }
        },
        Err(e) => {
            log::error!("error: {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}
