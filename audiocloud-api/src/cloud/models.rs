use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ModelFilter {
    pub manufacturer_is: Option<String>,
    pub name_contains:   Option<String>,
}
