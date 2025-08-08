use crate::schema::resolve_boxed_schema;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIModel {
    pub description: Option<String>,
    pub properties: IndexMap<String, String>,
}

impl From<openapi_kit_schema::Schema> for OpenAPIModel {
    fn from(value: openapi_kit_schema::Schema) -> Self {
        if let openapi_kit_schema::SchemaKind::Type(schema_type) = value.schema_kind {
            if let openapi_kit_schema::Type::Object(object) = schema_type {
                let mut properties = IndexMap::new();

                for (key, value) in object.properties {
                    properties.insert(key, resolve_boxed_schema(value));
                }

                return OpenAPIModel {
                    description: value.schema_data.description,
                    properties,
                };
            }
        }

        panic!("Only Object Type is supported for Models so far.");
    }
}
