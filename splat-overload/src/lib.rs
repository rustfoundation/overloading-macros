use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    ItemFn,
    FnArg,
    Pat,
    Result,
};

struct OverloadInput {
    functions: Vec<ItemFn>,
}

impl Parse for OverloadInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut functions = Vec::new();
        while !input.is_empty() {
            functions.push(input.parse::<ItemFn>()?);
        }
        Ok(OverloadInput { functions })
    }
}

#[proc_macro]
pub fn overload(input: TokenStream) -> TokenStream {
    let OverloadInput { functions } = parse_macro_input!(input as OverloadInput);

    
    let fn_name = &functions[0].sig.ident;

    
    let trait_name = quote::format_ident!(
        "{}Args",
        fn_name.to_string()
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_uppercase().next().unwrap() } else { c })
            .collect::<String>()
    );

    
    let mut impls = Vec::new();
    for func in &functions {
        for arg in &func.sig.inputs {
            if let FnArg::Typed(pat_type) = arg {
                let ty = &pat_type.ty;
                let block = &func.block;

                
                let arg_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let ident = &pat_ident.ident;
                    quote! { #ident }
                } else {
                    quote! { _arg }
                };

                impls.push(quote! {
                    impl #trait_name for (#ty,) {
                        fn call(self) {
                            let #arg_name = self.0;
                            #block
                        }
                    }
                });
            }
        }
    }

    
    let generated = quote! {
        trait #trait_name: std::marker::Tuple {
            fn call(self);
        }

        #(#impls)*

        fn #fn_name<T: #trait_name>(#[splat] args: T) {
            args.call()
        }
    };

    generated.into()
}

