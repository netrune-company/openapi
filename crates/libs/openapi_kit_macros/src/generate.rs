use std::fs::read_to_string;

use handlebars::Handlebars;
use openapi_kit_workspace::Workspace;
use proc_macro::TokenStream;
use syn::{Error, LitStr, parse::Parse, parse_macro_input, spanned::Spanned};

pub fn from_project(input: TokenStream) -> TokenStream {
    // Parse into tokens to a Reference.
    let reference = parse_macro_input!(input as Reference);

    // Load nearest Workspace.
    let workspace = match Workspace::load() {
        Ok(workspace) => workspace,
        Err(error) => {
            panic!("{error}");
        }
    };

    // Retrieve project and template from Config.
    let Some(project) = workspace.config.projects.get(&reference.project) else {
        panic!("Could not find project");
    };
    let Some(template) = project.templates.get(&reference.template) else {
        panic!("Could not find template");
    };

    // Retrieve template contents from path.
    let template_path = workspace.path.join(".openapi").join(&template.path);
    let Ok(template_content) = read_to_string(&template_path) else {
        panic!("Failed to read file at {}", template_path.display());
    };

    // Set fallback for schema path, and load the schema
    let schema_path = workspace.path.join(&project.schema_path);
    let Ok(schema) = openapi_kit_schema::load(&schema_path) else {
        panic!("Failed to load schema at {}", schema_path.display());
    };

    // Render the template
    let hbs = Handlebars::new();
    let Ok(output) = hbs.render_template(&template_content, &schema) else {
        panic!("Failed to render template");
    };

    // Return as a string literal
    let Ok(parsed) = output.parse() else {
        panic!("Failed to parse output");
    };

    parsed
}

/// A Reference to the project and template.
///
struct Reference {
    project: String,
    template: String,
}

impl Parse for Reference {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse input to a literal string
        let input = input.parse::<LitStr>()?.value();

        // Split string based on colon.
        let Some((project, template)) = input.split_once(":") else {
            return Err(Error::new(
                input.span(),
                "Expected pattern <project>:<template>",
            ));
        };

        // Ensure project and template references are not empty
        if project == "" {
            return Err(Error::new(input.span(), "Project name can not be empty"));
        }
        if template == "" {
            return Err(Error::new(input.span(), "Template name can not be empty"));
        }

        Ok(Reference {
            project: String::from(project),
            template: String::from(template),
        })
    }
}
