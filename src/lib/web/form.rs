use crate::domain::clip::field;
use rocket::form::FromForm;
use serde::Serialize;

pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires_at: field::ExpiresAt,
    pub password: field::Password,
}

#[derive(Debug, Serialize, FromForm)]
pub struct GetPasswordProtectedClip {
    pub password: field::Password,
}
