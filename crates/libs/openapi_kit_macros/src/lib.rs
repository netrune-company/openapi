mod generate;

use proc_macro::TokenStream;

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    generate::from_project(input)
}
