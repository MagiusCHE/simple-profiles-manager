use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
