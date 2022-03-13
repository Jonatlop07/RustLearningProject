use crate::data::DbId;
use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Constructor, Serialize, Deserialize)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ClipId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}