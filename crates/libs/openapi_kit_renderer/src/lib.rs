mod error;
mod schema;

use error::Error;
use openapi_kit_workspace::Workspace;
use schema::OpenAPI;
use tera::{Context, Tera};

pub struct Renderer {
    engine: Tera,
}

impl Renderer {
    pub fn new(workspace: &Workspace, _project: &str) -> Result<Self, Error> {
        let path = workspace
            .path
            .join(".openapi")
            .join("templates")
            .join("**")
            .join("*")
            .display()
            .to_string();

        let mut engine = Tera::new(path.as_ref())?;

        engine.register_filter("upper", filters::upper);
        engine.register_filter("pascal", filters::pascal);
        engine.register_filter("snake", filters::snake);
        engine.register_filter("camel", filters::camel);
        engine.register_filter("as_list", filters::as_list);

        Ok(Self { engine })
    }

    pub fn render(&self, template: &str, data: &OpenAPI) -> Result<String, Error> {
        let context = Context::from_serialize(data)?;
        Ok(self.engine.render(template, &context)?)
    }
}

mod filters {
    use convert_case::{Case, Casing};
    use std::collections::HashMap;
    use tera::{Error, Result, Value};

    pub fn as_list(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or_else(|| Error::msg("Value is not a string"))?;

        Ok(Value::String(format!("Vec<{text}>")))
    }

    pub fn upper(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or_else(|| Error::msg("Value is not a string"))?
            .to_case(Case::Upper);

        Ok(Value::String(text))
    }

    pub fn pascal(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or_else(|| Error::msg("Value is not a string"))?
            .to_case(Case::Pascal);

        Ok(Value::String(text))
    }

    pub fn snake(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or_else(|| Error::msg("Value is not a string"))?
            .to_case(Case::Snake);

        Ok(Value::String(text))
    }

    pub fn camel(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or_else(|| Error::msg("Value is not a string"))?
            .to_case(Case::Camel);

        Ok(Value::String(text))
    }
}
