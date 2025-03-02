use std::fs::read_to_string;

use handlebars::Handlebars;
use openapi_kit_workspace::Workspace;
use proc_macro::TokenStream;

pub fn from_file(path: &str) -> TokenStream {
    let Ok(workspace) = Workspace::load() else {
        panic!("Could not load workspace");
    };

    let path = workspace.path.join(".openapi").join(path);

    let Ok(content) = read_to_string(path) else {
        panic!("Failed to read file");
    };

    generate(content.as_str(), workspace)
}

pub fn from_template(template: &str) -> TokenStream {
    let Ok(workspace) = Workspace::load() else {
        panic!("Could not load workspace");
    };

    generate(template, workspace)
}

fn generate(template: &str, workspace: Workspace) -> TokenStream {
    // Set fallback for schema path, and load the schema
    let schema_path = workspace.path.join("openapi.yaml");
    let Ok(schema) = openapi_kit_schema::load(&schema_path) else {
        panic!("Failed to load schema at {}", schema_path.display());
    };

    // Render the template
    let hbs = Handlebars::new();
    let Ok(output) = hbs.render_template(&template, &schema) else {
        panic!("Failed to render template");
    };

    // Return as a string literal
    let Ok(parsed) = output.parse() else {
        panic!("Failed to parse output");
    };

    parsed
}
