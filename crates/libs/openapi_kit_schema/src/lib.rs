mod error;

use std::path::Path;

pub use error::Error;

pub use openapiv3::*;

pub fn load<P: AsRef<Path>>(path: P) -> Result<OpenAPI, Error> {
    let file = std::fs::read_to_string(path).map_err(Error::Io)?;
    serde_yml::from_str(&file).map_err(Error::Serde)
}
