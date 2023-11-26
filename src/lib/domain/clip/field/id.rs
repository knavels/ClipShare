use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct Id(DbId);

impl Id {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for Id {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
