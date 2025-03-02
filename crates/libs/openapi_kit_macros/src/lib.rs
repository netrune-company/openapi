mod generate;

use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

/// Generate code from a file
///
#[proc_macro]
pub fn generate_from_file(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr).value();
    generate::from_file(path.as_str())
}

/// Generate code from a string
///
#[proc_macro]
pub fn generate_from_template(input: TokenStream) -> TokenStream {
    let template = parse_macro_input!(input as LitStr).value();
    generate::from_template(template.as_str())
}
