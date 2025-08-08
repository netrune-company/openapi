use crate::schema::model::OpenAPIModel;
use crate::schema::path::OpenAPIPath;
use crate::schema::resolve_boxed_schema;
use indexmap::IndexMap;
use openapi_kit_schema::{ReferenceOr, SchemaKind, Type};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPI {
    pub paths: IndexMap<String, OpenAPIPath>,
    pub models: IndexMap<String, OpenAPIModel>,
}

impl From<openapi_kit_schema::OpenAPI> for OpenAPI {
    fn from(value: openapi_kit_schema::OpenAPI) -> Self {
        OpenAPI {
            paths: convert_paths(value.paths),
            models: convert_components(value.components),
        }
    }
}

fn convert_paths(paths: openapi_kit_schema::Paths) -> IndexMap<String, OpenAPIPath> {
    let mut output = IndexMap::new();

    for (path, ref_or_item) in paths {
        if let ReferenceOr::Item(item) = ref_or_item {
            output.insert(path, item.into());
        }
    }

    output
}

fn convert_components(
    components: Option<openapi_kit_schema::Components>,
) -> IndexMap<String, OpenAPIModel> {
    let mut output = IndexMap::new();

    if let Some(components) = components {
        for (name, ref_or_item) in components.schemas {
            if let ReferenceOr::Item(item) = ref_or_item {
                let data = item.schema_data;
                if let SchemaKind::Type(Type::Object(object)) = item.schema_kind {
                    let mut properties = IndexMap::new();

                    for (property, ref_or_schema) in object.properties {
                        properties.insert(property, resolve_boxed_schema(ref_or_schema));
                    }

                    output.insert(
                        name,
                        OpenAPIModel {
                            description: data.description,
                            properties,
                        },
                    );
                }
            }
        }
    }

    output
}
