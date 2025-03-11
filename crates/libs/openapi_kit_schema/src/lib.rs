mod error;
mod resolve;

use std::path::Path;

pub use error::Error;

pub type OpenApiSchema = openapiv3::OpenAPI;

pub fn load<P: AsRef<Path>>(path: P) -> Result<OpenApiSchema, Error> {
    let file = std::fs::read_to_string(path).map_err(Error::Io)?;
    serde_yaml::from_str(&file).map_err(Error::Serde)
}
