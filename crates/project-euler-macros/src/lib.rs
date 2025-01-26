use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn gen_mods(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = lit.base10_parse::<u16>().unwrap();
    let idents = (1..=value).map(|n| {
        let mod_name = format!("p{:04}", n);
        syn::Ident::new(&mod_name, lit.span())
    });

    quote! {
        #( pub mod #idents; )*
    }
    .into()
}
