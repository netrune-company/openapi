use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPI {
    #[serde(with = "indexmap::map::serde_seq")]
    paths: IndexMap<String, OpenAPIPath>,
    #[serde(with = "indexmap::map::serde_seq")]
    models: IndexMap<String, OpenAPIModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    post: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    get: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch: Option<OpenAPIOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<OpenAPIOperation>,
}

impl IntoIterator for OpenAPIPath {
    type Item = (&'static str, OpenAPIOperation);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("post", self.post),
            ("get", self.get),
            ("put", self.put),
            ("patch", self.patch),
            ("delete", self.delete),
        ]
        .into_iter()
        .filter_map(|(method, maybe_op)| maybe_op.map(|op| (method, op)))
        .collect::<Vec<_>>()
        .into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIOperation {
    operation_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIModel {
    description: Option<String>,
    #[serde(with = "indexmap::map::serde_seq")]
    properties: IndexMap<String, String>,
}

pub trait MaybeFrom<T>: Sized {
    fn maybe_from(value: T) -> Option<Self>;
}

impl From<openapi_kit_schema::OpenAPI> for OpenAPI {
    fn from(value: openapi_kit_schema::OpenAPI) -> Self {
        let mut paths = IndexMap::new();
        let mut models = IndexMap::new();

        // Populate paths.
        value.paths.paths.into_iter().for_each(
            |(endpoint, ref_or_path_item)| match ref_or_path_item {
                openapi_kit_schema::ReferenceOr::Reference { reference: _ } => (),
                openapi_kit_schema::ReferenceOr::Item(path_item) => {
                    paths.insert(endpoint, OpenAPIPath::from(path_item));
                }
            },
        );

        // Populate models.
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

impl From<openapi_kit_schema::PathItem> for OpenAPIPath {
    fn from(value: openapi_kit_schema::PathItem) -> Self {
        OpenAPIPath {
            post: value.post.and_then(OpenAPIOperation::maybe_from),
            get: value.get.and_then(OpenAPIOperation::maybe_from),
            put: value.put.and_then(OpenAPIOperation::maybe_from),
            patch: value.patch.and_then(OpenAPIOperation::maybe_from),
            delete: value.delete.and_then(OpenAPIOperation::maybe_from),
        }
    }
}

impl MaybeFrom<openapi_kit_schema::Operation> for OpenAPIOperation {
    fn maybe_from(value: openapi_kit_schema::Operation) -> Option<Self> {
        if let Some(operation_id) = value.operation_id {
            return Some(OpenAPIOperation { operation_id });
        }
        None
    }
}

impl MaybeFrom<openapi_kit_schema::Schema> for OpenAPIModel {
    fn maybe_from(value: openapi_kit_schema::Schema) -> Option<Self> {
        match value.schema_kind {
            openapi_kit_schema::SchemaKind::Type(openapi_kit_schema::Type::Object(object_type)) => {
                let mut model = OpenAPIModel {
                    description: value.schema_data.description,
                    properties: IndexMap::new(),
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
                                        openapi_kit_schema::Type::String(s) => {
                                            match s.format {
                                                openapi_kit_schema::VariantOrUnknownOrEmpty::Unknown(format) => {
                                                    if format == "uuid" {
                                                        String::from("uuid::Uuid")
                                                    } else {
                                                        String::from("String")
                                                    }
                                                },
                                                _ => String::from("String")
                                            }
                                            
                                        }
                                        openapi_kit_schema::Type::Number(number_type) => {
                                            match number_type.format {
                                                openapi_kit_schema::VariantOrUnknownOrEmpty::Item(_) => {
                                                    // The format is either double or float here.
                                                    String::from("f64")
                                                },
                                                _ => String::from("i64")
                                            }
                                        },
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
