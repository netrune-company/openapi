pub type OpenApiSchema = openapiv3::OpenAPI;

pub enum Error {
    Io(std::io::Error),
    Serde(serde_yaml::Error),
}

pub fn load(path: &str) -> Result<OpenApiSchema, Error> {
    let file = std::fs::read_to_string(path).map_err(|e| Error::Io(e))?;
    serde_yaml::from_str(&file).map_err(|e| Error::Serde(e))
}
