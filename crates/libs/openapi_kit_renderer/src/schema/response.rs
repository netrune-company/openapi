use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIResponse {
    pub kind: String,
    pub media: String,
}
