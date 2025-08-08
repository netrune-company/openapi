use serde::Deserialize;
use serde::Serialize;

use crate::schema::operation::OpenAPIOperation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OpenAPIOperation>,
}

impl From<openapi_kit_schema::PathItem> for OpenAPIPath {
    fn from(value: openapi_kit_schema::PathItem) -> Self {
        OpenAPIPath {
            post: value.post.map(Into::into),
            get: value.get.map(Into::into),
            put: value.put.map(Into::into),
            patch: value.patch.map(Into::into),
            delete: value.delete.map(Into::into),
        }
    }
}
