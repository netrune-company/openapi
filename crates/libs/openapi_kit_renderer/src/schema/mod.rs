mod model;
mod openapi;
mod operation;
mod parameter;
mod path;
mod request;
mod response;

pub use openapi::OpenAPI;
use openapi_kit_schema::{ReferenceOr, SchemaKind, Type, VariantOrUnknownOrEmpty};

pub fn resolve_schema(ref_or_schema: ReferenceOr<openapi_kit_schema::Schema>) -> String {
    reference_or(ref_or_schema, |schema| {
        resolve_boxed_schema(ReferenceOr::Item(Box::new(schema)))
    })
}

pub fn resolve_boxed_schema(
    ref_or_boxed_schema: ReferenceOr<Box<openapi_kit_schema::Schema>>,
) -> String {
    reference_or(ref_or_boxed_schema, |schema| {
        if let SchemaKind::Type(schema_type) = schema.schema_kind {
            return match schema_type {
                Type::String(string) => {
                    if let VariantOrUnknownOrEmpty::Unknown(format) = string.format {
                        if format == "uuid" {
                            return String::from("uuid::Uuid");
                        }
                    }

                    String::from("String")
                }
                Type::Boolean(_) => String::from("bool"),
                Type::Array(array) => {
                    if let Some(kind) = array.items.map(|item| resolve_boxed_schema(item)) {
                        return format!("Vec< {} >", kind);
                    };

                    panic!("Could not resolve array.items");
                }
                Type::Object(_) => {
                    panic!("Could not resolve object.");
                }
                Type::Number(_) => "f64".to_string(),
                Type::Integer(_) => "i64".to_string(),
            };
        }

        panic!("Only types are supported.")
    })
}

pub fn reference_or<F, T>(ref_or_item: ReferenceOr<T>, callback: F) -> String
where
    F: FnOnce(T) -> String,
{
    match ref_or_item {
        ReferenceOr::Reference { reference } => reference
            .split("/")
            .last()
            .expect("Reference is not formatted correctly")
            .to_string(),
        ReferenceOr::Item(item) => callback(item),
    }
}
