use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn gen_mods(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = lit.base10_parse::<u16>().unwrap();
    let idents: Vec<_> = (1..=value)
        .map(|n| {
            let mod_name = format!("p{:04}", n);
            (n, syn::Ident::new(&mod_name, lit.span()))
        })
        .collect();

    let mut modules = Vec::new();
    for (_, ident) in idents.iter() {
        let module = quote! {
            mod #ident;
        };
        modules.push(module);
    }

    let mut match_arms = Vec::new();
    for (n, ident) in idents.iter() {
        let arm = quote! {
            #n => #ident::solve(),
        };
        match_arms.push(arm);
    }

    let mut solutions = Vec::new();
    for (_, ident) in idents.iter() {
        let solution = quote! {
            solutions.push(#ident::solve());
        };
        solutions.push(solution);
    }

    let functions = quote! {
        /// Solve problem number `n`.
        pub fn solve(n: u16) -> impl ::std::fmt::Display {
            match n {
                #(#match_arms)*
                _ => unimplemented!(),
            }
        }

        /// Solve all problems.
        pub fn solve_all() -> impl ::std::iter::Iterator<Item = impl ::std::fmt::Display> {
            let mut solutions = ::std::vec::Vec::new();
            #(#solutions)*

            solutions.into_iter()
        }
    };

    quote! {
        #(#modules)*
        /// The highest numbered problem that has been solved.
        pub const PMAX: u16 = #value;
        #functions
    }
    .into()
}
