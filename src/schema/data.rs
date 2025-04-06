use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SourceConfig {
    #[serde(rename = "api")]
    Api {
        _id: String,
        host: String,
        path: String,
        method: String,
        response_property: String,
    },
    #[serde(rename = "web")]
    Web {
        _id: String,
        host: String,
        path: String,
        dom_selectors: Vec<DomSelector>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomSelector {
    pub method: String, // "id", "class", "tag", etc.
    pub value: String,
}
