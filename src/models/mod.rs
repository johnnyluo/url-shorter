use mysql::chrono;

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct UrlMeta {
    pub id: u64,
    pub user_id: u64,
    pub target_url: String,
    pub shortened_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}