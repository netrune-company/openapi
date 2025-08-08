use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIParameter {
    pub r#in: String,
    pub name: String,
    pub kind: String,
}
