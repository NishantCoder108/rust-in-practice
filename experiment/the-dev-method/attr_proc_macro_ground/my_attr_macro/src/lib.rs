use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Expr);

    let name = &input.sig.ident;
    let block = &input.block;

    let expanded = quote! {
        fn #name() {
            println!("Function {} called!", stringify!(#name));
            #block
        }
    };

    TokenStream::from(expanded)
}
