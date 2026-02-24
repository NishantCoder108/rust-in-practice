use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(HeyNishant)]
pub fn hey_nishant_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let expanded = quote! {
        impl #name {
            pub fn hey() {
                println!("Hey Nishant!");
            }
        }
    };

    TokenStream::from(expanded)
}
