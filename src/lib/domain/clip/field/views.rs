use crate::domain::clip::ClipError;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Serialize, Deserialize)]
pub struct Views(u64);

impl Views {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
