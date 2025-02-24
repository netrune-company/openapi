#![feature(proc_macro_span)]
use proc_macro::TokenStream;

use handlebars::Handlebars;

#[proc_macro]
pub fn openapi(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();

    // Get spans of the first and last tokens
    let Some(first_span) = tokens.first().map(|t| t.span()) else {
        return TokenStream::new();
    };
    let Some(last_span) = tokens.last().map(|t| t.span()) else {
        return TokenStream::new();
    };

    // Join spans to get the full range
    let Some(combined_span) = first_span.join(last_span) else {
        return TokenStream::new();
    };

    // Extract raw source code between the spans
    let Some(source) = combined_span.source_text() else {
        return TokenStream::new();
    };

    // Load OpenAPI schema
    let schema = openapi_kit_schema::load("openapi.yaml");

    // Render the template
    let hbs = Handlebars::new();
    let Ok(output) = hbs.render_template(&source, &schema) else {
        return TokenStream::new();
    };

    // Return as a string literal
    output.parse().unwrap()
}
