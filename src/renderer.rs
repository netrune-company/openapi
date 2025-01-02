use std::collections::HashMap;

use anyhow::Ok;
use tera::{Context, Tera};

use crate::schema::{OpenAPI, PathItemObject, SchemaObject};

pub struct Renderer {
    tera: Tera,
    schema: OpenAPI,
}

impl Renderer {
    pub fn new(openapi_schema: OpenAPI) -> anyhow::Result<Self> {
        let mut tera = Tera::new(".openapi/flavours/**/*.tera")?;

        tera.register_filter("pascal_case", filter::pascal_case);
        tera.register_filter("snake_case", filter::snake_case);
        tera.register_filter("camel_case", filter::camel_case);

        Ok(Renderer {
            tera,
            schema: openapi_schema,
        })
    }

    pub fn render(
        &self,
        path: &str,
        data: &HashMap<String, String>,
        schema: Option<SchemaObject>,
        path_item: Option<PathItemObject>,
    ) -> anyhow::Result<String> {
        let mut context = Context::new();

        // Inject openapi schema.
        context.insert("schema", &self.schema);

        for (key, value) in data {
            context.insert(key, value);
        }

        // TODO: This has to be redone. The schema object doesn't seem to follow the standard, and doesn't contain the expected values.
        if let Some(schema) = schema {
            context.insert("discriminator", &schema.discriminator);
            context.insert("example", &schema.example);
            context.insert("external_docs", &schema.external_docs);
            context.insert("xml", &schema.xml);
        }

        if let Some(path_item) = path_item {
            context.insert("methods", &path_item);
        }

        Ok(self.tera.render(&format!("{path}.tera"), &context)?)
    }

    pub fn render_str(
        &mut self,
        source: &str,
        data: &HashMap<String, String>,
    ) -> anyhow::Result<String> {
        let mut context = Context::new();

        context.insert("schema", &self.schema);

        for (key, value) in data {
            context.insert(key, value);
        }

        Ok(self.tera.render_str(source, &context)?)
    }
}

mod filter {
    use convert_case::{Case, Casing};
    use std::collections::HashMap;
    use tera::{Result, Value};

    pub fn pascal_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        Ok(Value::String(
            value.as_str().unwrap_or("").to_case(Case::Pascal),
        ))
    }

    pub fn snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        Ok(Value::String(
            value.as_str().unwrap_or("").to_case(Case::Snake),
        ))
    }

    pub fn camel_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        Ok(Value::String(
            value.as_str().unwrap_or("").to_case(Case::Camel),
        ))
    }
}
