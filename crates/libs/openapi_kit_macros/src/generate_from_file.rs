use std::fs::read_to_string;

use handlebars::Handlebars;
use proc_macro::TokenStream;
use syn::{LitStr, Token, parse::Parse, parse_macro_input};

pub fn parse(input: TokenStream) -> TokenStream {
    // Parse input tokens to get the template and schema paths
    let args = parse_macro_input!(input as FromFileArgs);

    // Read the template file
    let Ok(content) = read_to_string(args.template.value()) else {
        panic!("Failed to read file");
    };

    // Set fallback for schema path, and load the schema
    let schema_path = args
        .schema
        .map(|s| s.value())
        .unwrap_or(String::from("openapi.yaml"));
    let Ok(schema) = openapi_kit_schema::load(schema_path.as_ref()) else {
        panic!("Failed to load schema at {}", schema_path);
    };

    // Render the template
    let hbs = Handlebars::new();
    let Ok(output) = hbs.render_template(&content, &schema) else {
        panic!("Failed to render template");
    };

    // Return as a string literal
    let Ok(parsed) = output.parse() else {
        panic!("Failed to parse output");
    };

    parsed
}

struct FromFileArgs {
    template: LitStr,
    schema: Option<LitStr>,
}

impl Parse for FromFileArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse the template path
        let template = input.parse()?;

        // Parse an optional schema path
        let schema = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse()?
        } else {
            None
        };

        // Ensure there are no more tokens
        if !input.is_empty() {
            return Err(input.error("unexpected token"));
        }

        Ok(FromFileArgs { template, schema })
    }
}
