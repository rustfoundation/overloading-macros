use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, ItemFn, Pat, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

enum OverloadInput {
    Functions(Vec<ItemFn>),
    Methods {
        self_ty: syn::Ident,
        functions: Vec<ItemFn>,
    },
}

impl Parse for OverloadInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![impl]) {
            let item_impl: syn::ItemImpl = input.parse()?;

            let self_ty = match &*item_impl.self_ty {
                syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.clone(),
                _ => panic!("overload! impl block must use a plain type name"),
            };

            let functions = item_impl
                .items
                .into_iter()
                .map(|item| match item {
                    syn::ImplItem::Fn(impl_fn) => syn::ItemFn {
                        attrs: impl_fn.attrs,
                        vis: impl_fn.vis,
                        sig: impl_fn.sig,
                        block: Box::new(impl_fn.block),
                    },
                    _ => panic!("overload! impl block may only contain fn items"),
                })
                .collect();

            Ok(OverloadInput::Methods { self_ty, functions })
        } else {
            let mut functions = Vec::new();
            while !input.is_empty() {
                functions.push(input.parse::<ItemFn>()?);
            }
            Ok(OverloadInput::Functions(functions))
        }
    }
}

fn trait_name_for(fn_name: &syn::Ident) -> syn::Ident {
    quote::format_ident!(
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
    )
}

fn collect_args(
    func: &ItemFn,
) -> (
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
) {
    let mut arg_types = Vec::new();
    let mut arg_names = Vec::new();
    let mut arg_indices = Vec::new();

    let mut index = 0;
    for arg in &func.sig.inputs {
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

            let idx = syn::Index::from(index);
            arg_indices.push(quote! { self.#idx });
            index += 1;
        }
    }

    (arg_types, arg_names, arg_indices)
}

fn output_ty_for(func: &ItemFn) -> proc_macro2::TokenStream {
    match &func.sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    }
}

fn tuple_ty_for(arg_types: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    if arg_types.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#arg_types),*,) }
    }
}

fn generate_free_functions(functions: Vec<ItemFn>) -> TokenStream {
    let fn_name = &functions[0].sig.ident;
    let trait_name = trait_name_for(fn_name);

    let mut impls = Vec::new();
    for func in &functions {
        let (arg_types, arg_names, arg_indices) = collect_args(func);
        let output_ty = output_ty_for(func);
        let tuple_ty = tuple_ty_for(&arg_types);
        let block = &func.block;

        impls.push(quote! {
            impl #trait_name for #tuple_ty {
                type Output = #output_ty;
                fn call(self) -> Self::Output {
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

        fn #fn_name<T: #trait_name>(#[rustc_splat] args: T) -> T::Output {
            args.call()
        }
    };

    generated.into()
}

fn generate_methods(self_ty: syn::Ident, functions: Vec<ItemFn>) -> TokenStream {
    let fn_name = &functions[0].sig.ident;
    let trait_name = trait_name_for(fn_name);

    let (is_ref, is_mut) = {
        let first_receiver = match functions[0].sig.inputs.first() {
            Some(FnArg::Receiver(r)) => r,
            _ => panic!("overload! methods must take self"),
        };
        for func in &functions {
            match func.sig.inputs.first() {
                Some(FnArg::Receiver(r)) => {
                    let same = r.reference.is_some() == first_receiver.reference.is_some()
                        && r.mutability.is_some() == first_receiver.mutability.is_some();
                    if !same {
                        panic!(
                            "all overloads must use the same receiver kind (&self, &mut self, or self)"
                        );
                    }
                }
                _ => panic!("overload! methods must take self"),
            }
        }
        (
            first_receiver.reference.is_some(),
            first_receiver.mutability.is_some(),
        )
    };

    let this_generic_ty = match (is_ref, is_mut) {
        (true, true) => quote! { &mut R },
        (true, false) => quote! { &R },
        (false, _) => quote! { R },
    };

    let this_concrete_ty = match (is_ref, is_mut) {
        (true, true) => quote! { &mut #self_ty },
        (true, false) => quote! { &#self_ty },
        (false, _) => quote! { #self_ty },
    };

    let mut impls = Vec::new();
    let mut hidden_methods = Vec::new();

    for (i, func) in functions.iter().enumerate() {
        let (arg_types, arg_names, arg_indices) = collect_args(func);
        let output_ty = output_ty_for(func);
        let tuple_ty = tuple_ty_for(&arg_types);
        let block = &func.block;
        let func_receiver = match func.sig.inputs.first() {
            Some(FnArg::Receiver(r)) => quote! { #r },
            _ => panic!("overload! methods must take self"),
        };

        let hidden_name = quote::format_ident!("__{}_impl_{}", fn_name, i);

        hidden_methods.push(quote! {
            fn #hidden_name(#func_receiver, #(#arg_names: #arg_types),*) -> #output_ty {
                #block
            }
        });

        impls.push(quote! {
            impl #trait_name<#self_ty> for #tuple_ty {
                type Output = #output_ty;
                fn call(self, this: #this_concrete_ty) -> Self::Output {
                    #(let #arg_names = #arg_indices;)*
                    this.#hidden_name(#(#arg_names),*)
                }
            }
        });
    }

    let receiver = match functions[0].sig.inputs.first() {
        Some(FnArg::Receiver(r)) => quote! { #r },
        _ => unreachable!(),
    };

    let generated = quote! {
        trait #trait_name<R>: std::marker::Tuple {
            type Output;
            fn call(self, this: #this_generic_ty) -> Self::Output;
        }

        #(#impls)*

        impl #self_ty {
            #(#hidden_methods)*

            fn #fn_name<T: #trait_name<Self>>(#receiver, #[rustc_splat] args: T) -> T::Output {
                args.call(self)
            }
        }
    };

    generated.into()
}

#[proc_macro]
pub fn overload(input: TokenStream) -> TokenStream {
    match parse_macro_input!(input as OverloadInput) {
        OverloadInput::Functions(functions) => generate_free_functions(functions),
        OverloadInput::Methods { self_ty, functions } => generate_methods(self_ty, functions),
    }
}
