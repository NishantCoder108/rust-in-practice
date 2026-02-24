use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Hello)]
// "Hello" here is just the name of the custom derive macro. When you write #[derive(Hello)] on a struct (like in your main.rs), Rust invokes this macro.
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // `ident` is a field of the `DeriveInput` struct provided by `syn` crate.
    // It represents the name (identifier) of the struct, enum, or union for which the macro is being derived.
    // So, for `#[derive(Hello)] struct User;`, `ast.ident` would be `User`.
    let name = ast.ident;

    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello from macro!");
            }
        }
    };

    TokenStream::from(expanded)
}
