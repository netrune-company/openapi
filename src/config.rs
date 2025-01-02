use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

#[derive(Deserialize)]
pub struct AppConfig {
    pub schema: String,
    pub flavour: String,
    pub inputs: HashMap<String, String>,
}

impl AppConfig {
    pub fn new(schema: Option<String>, flavour: Option<String>, inputs: Vec<String>) -> Self {
        let mut input_map = HashMap::new();

        let mut iterator = inputs.into_iter();

        while let Some(key) = iterator.next() {
            if let Some(value) = iterator.next() {
                input_map.insert(key, value);
            }
        }

        AppConfig {
            schema: schema.unwrap_or(String::from("openapi.yaml")),
            flavour: flavour.unwrap_or(String::from("default")),
            inputs: input_map,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FlavourConfig {
    pub version: Option<String>,
    pub language: String,
    pub inputs: Option<HashMap<String, FlavourInput>>,
    pub templates: Vec<TemplateManifest>,
}

#[derive(Deserialize, Debug)]
pub struct FlavourInput {
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TemplateManifest {
    pub name: String,
    pub input: String,
    pub output_dir: String,
    pub output: String,
    pub iteration: Option<IterationKind>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IterationKind {
    #[serde(rename = "schemas")]
    Schemas,
    #[serde(rename = "paths")]
    Paths,
}

impl FlavourConfig {
    pub fn load(flavour: &str) -> anyhow::Result<FlavourConfig> {
        let file = File::open(format!(".openapi/flavours/{}/config.yaml", flavour))
            .map_err(|error| anyhow!("Could not load flavour config. {error}"))?;
        serde_yaml::from_reader(file).map_err(Into::into)
    }
}
