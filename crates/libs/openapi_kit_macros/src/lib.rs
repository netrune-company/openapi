mod generate_from_file;

use proc_macro::TokenStream;

/// Generate code from a template file
///
#[proc_macro]
pub fn from_file(input: TokenStream) -> TokenStream {
    generate_from_file::parse(input)
}
