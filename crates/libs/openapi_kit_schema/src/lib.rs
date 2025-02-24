pub type OpenApiSchema = openapiv3::OpenAPI;

pub fn load(path: &str) -> OpenApiSchema {
    let file = std::fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&file).unwrap()
}
