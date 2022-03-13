use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Clone, Constructor, Debug, Serialize, Deserialize)]
pub struct Hits(u32);

impl Hits {
    pub fn into_inner(self) -> u32 {
        self.0
    }
}
