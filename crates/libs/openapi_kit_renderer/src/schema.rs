use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPI {
    paths: HashMap<String, OpenAPIPath>,
    models: HashMap<String, OpenAPIModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIPath {
    post: Option<OpenAPIOperation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIOperation {
    operation_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIModel {
    properties: HashMap<String, String>,
}

pub trait MaybeFrom<T>: Sized {
    fn maybe_from(value: T) -> Option<Self>;
}

impl From<openapi_kit_schema::OpenAPI> for OpenAPI {
    fn from(value: openapi_kit_schema::OpenAPI) -> Self {
        let paths = HashMap::new();
        let mut models = HashMap::new();

        if let Some(components) = value.components {
            components
                .schemas
                .into_iter()
                .for_each(|(key, ref_or_schema)| match ref_or_schema {
                    openapi_kit_schema::ReferenceOr::Reference { reference: _ } => (),
                    openapi_kit_schema::ReferenceOr::Item(schema) => {
                        if let Some(schema) = OpenAPIModel::maybe_from(schema) {
                            models.insert(key, schema);
                        }
                    }
                });
        }

        OpenAPI { paths, models }
    }
}

impl MaybeFrom<openapi_kit_schema::Schema> for OpenAPIModel {
    fn maybe_from(value: openapi_kit_schema::Schema) -> Option<Self> {
        match value.schema_kind {
            openapi_kit_schema::SchemaKind::Type(openapi_kit_schema::Type::Object(object_type)) => {
                let mut model = OpenAPIModel {
                    properties: HashMap::new(),
                };

                let required = object_type.required.clone();

                object_type
                    .properties
                    .into_iter()
                    .for_each(|(key, ref_or_property)| {
                        let kind = match ref_or_property {
                            openapi_kit_schema::ReferenceOr::Reference { reference } => {
                                reference.split("/").last().map(Into::into)
                            }
                            openapi_kit_schema::ReferenceOr::Item(property) => {
                                if let openapi_kit_schema::SchemaKind::Type(property_kind) =
                                    property.schema_kind
                                {
                                    Some(match property_kind {
                                        openapi_kit_schema::Type::String(_) => {
                                            String::from("String")
                                        }
                                        openapi_kit_schema::Type::Number(_) => String::from("i64"),
                                        openapi_kit_schema::Type::Integer(_) => String::from("i64"),
                                        openapi_kit_schema::Type::Array(array_type) => {
                                            let mut builder = String::from("Vec<");
                                            if let Some(
                                                openapi_kit_schema::ReferenceOr::Reference {
                                                    reference,
                                                },
                                            ) = array_type.items
                                            {
                                                if let Some(kind) = reference.split("/").last() {
                                                    builder.push_str(kind);
                                                }
                                            }

                                            builder.push('>');

                                            builder
                                        }
                                        openapi_kit_schema::Type::Boolean(_) => {
                                            String::from("bool")
                                        }
                                        _ => String::from("Value"),
                                    })
                                } else {
                                    None
                                }
                            }
                        };

                        if let Some(kind) = kind {
                            let mut builder = String::new();

                            let is_required = required.iter().any(|p| p == &key);

                            if !is_required {
                                builder.push_str("Option<");
                            }

                            builder.push_str(&kind);

                            if !is_required {
                                builder.push('>');
                            }

                            model.properties.insert(key, builder);
                        }
                    });

                Some(model)
            }
            _ => None,
        }
    }
}

/*

{% for (name, model) in models %}
pub struct {{ name }} {
    {% for (property, kind) in model.properties  %}
    pub {{ property }}: {{ kind }},
    {% endfor %}
}
{% endfor %}
*/
