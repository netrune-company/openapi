pub use openapi_kit_macros::generate_from_file as from_file;
pub use openapi_kit_macros::generate_from_template as from_template;
pub use openapi_kit_schema as schema;

// All modules related to generating code from OpenAPI schema files
pub mod generate {
    pub use super::from_file;
    pub use super::from_template;
}
