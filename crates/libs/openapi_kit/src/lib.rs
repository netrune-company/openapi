pub use openapi_kit_macros as macros;
pub use openapi_kit_schema as schema;

pub mod prelude {
    pub use crate::macros::*;
    pub use crate::schema::*;
}
