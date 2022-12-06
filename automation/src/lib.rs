// This library is used to create several procedural macros that will automatically import specific modules at any given point in my AoC system.
// They will be based off files of the format "dayXX.rs" in the same directory as main.rs.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn include_days_with_paths(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut expanded = quote! {};

    // List all files in the current directory.
    let files = fs::read_dir("./").unwrap();

    // Use a loop to generate code for loading solution modules.
    for file in files {
        let file_name = file.unwrap().file_name();
        let file_name_str = file_name.to_str().unwrap();

        // Check if the file name matches the "dayXX.rs" pattern.
        if file_name_str.starts_with("day") && file_name_str.ends_with(".rs") {
            let module_name = file_name_str.replace(".rs", "");
            let module_path = format!("../src/{}", file_name_str);

            expanded.extend(quote! {
                #[path = #module_path]
                mod #module_name;
            });
        }
    }

    TokenStream::from(expanded)
}
