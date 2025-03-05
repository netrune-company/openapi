use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub projects: HashMap<String, Project>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub language: String,
    #[serde(default = "schema_path_default")]
    pub schema_path: String,
    pub templates: HashMap<String, Template>,
}

fn schema_path_default() -> String {
    String::from("openapi.yaml")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub path: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;
        let config = serde_yml::from_reader(file)?;

        Ok(config)
    }
}
