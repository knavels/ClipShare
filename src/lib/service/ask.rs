use crate::domain::clip::field;
use crate::ShortCode;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub short_code: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(short_code: &str) -> Self {
        Self {
            short_code: ShortCode::from(short_code),
            password: field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(short_code: ShortCode) -> Self {
        Self {
            short_code,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(short_code: &str) -> Self {
        Self::from_raw(short_code)
    }
}
