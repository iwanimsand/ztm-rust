use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {
    /*
    -> This constructor is created by derive_more::Constructor with the Constructor macro

    pub fn new(data: u64) -> Self {
        Self(data)
    }
     */
    pub fn into_inner(self) -> u64 {
        self.0
    }
}