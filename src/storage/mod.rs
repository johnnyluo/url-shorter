use crate::models;
use std::error::Error;

pub fn SaveToDb() -> Result<models::UrlMeta, Error> {
    Ok(models::UrlMeta {
        id: "123".to_string(),
        user_id: "123".to_string(),
        target_url: "123".to_string(),
    })
}
