mod error;

use error::Error;
use openapi_kit_schema::OpenApiSchema;
use openapi_kit_workspace::Workspace;
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

        let engine = Tera::new(path.as_ref())?;

        Ok(Self { engine })
    }

    pub fn render(&self, template: &str, data: &OpenApiSchema) -> Result<String, Error> {
        let context = Context::from_serialize(data)?;
        Ok(self.engine.render(template, &context)?)
    }
}
