use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AmazonSettings {
    pub enabled: bool,
    pub prepend: bool,
    pub location: Option<String>,
}
