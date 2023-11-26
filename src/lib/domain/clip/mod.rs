use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod field;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("invalid password: {0}")]
    InvalidPassword(String),

    #[error("invalid title: {0}")]
    InvalidTitle(String),

    #[error("emoty content")]
    EmptyContent,

    #[error("invalid date: {0}")]
    InvalidDate(String),

    #[error("invalid parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),

    #[error("views parse error: {0}")]
    Views(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub id: field::Id,
    pub short_code: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub created_at: field::CreatedAt,
    pub expires_at: field::ExpiresAt,
    pub password: field::Password,
    pub views: field::Views,
}
