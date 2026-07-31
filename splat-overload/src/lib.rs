use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, ItemFn, Pat, Result,
    parse::{Parse, ParseStream},
    parse_macro_input,
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
        fn_name
            .to_string()
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 {
                c.to_uppercase().next().unwrap()
            } else {
                c
            })
            .collect::<String>()
    );

    let mut impls = Vec::new();
    for func in &functions {
        // Collect All arguments and names
        let mut arg_types = Vec::new();
        let mut arg_names = Vec::new();
        let mut arg_indices = Vec::new();
        let block = &func.block;
        for (i, arg) in func.sig.inputs.iter().enumerate() {
            if let FnArg::Typed(pat_type) = arg {
                let ty = &pat_type.ty;
                arg_types.push(quote! { #ty });

                let arg_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let ident = &pat_ident.ident;
                    quote! { #ident }
                } else {
                    quote! { _arg }
                };
                arg_names.push(arg_name);

                let index = syn::Index::from(i);
                arg_indices.push(quote! { self.#index });
            }
        }

        let output_ty = match &func.sig.output {
            syn::ReturnType::Default => quote! { () },
            syn::ReturnType::Type(_, ty) => quote! { #ty },
        };

        // Check if there are no arguments
        let tuple_ty = if arg_types.is_empty() {
            quote! { () }
        } else {
            quote! { (#(#arg_types),*,) }
        };

        impls.push(quote! {
            impl #trait_name for #tuple_ty {
                type Output = #output_ty;
                fn call(self) -> Self::Output{
                    #(let #arg_names = #arg_indices;)*
                    #block
                }
            }
        });
    }
    let generated = quote! {
        trait #trait_name: std::marker::Tuple {
            type Output;
            fn call(self) -> Self::Output;
        }

        #(#impls)*

        fn #fn_name<T: #trait_name>(#[rustc_splat] args: T) -> T::Output{
            args.call()
        }
    };

    generated.into()
}
