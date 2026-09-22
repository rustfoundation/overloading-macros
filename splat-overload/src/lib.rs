//! A [Rust Foundation *experiment*](https://rustfoundation.org/media/experimenting-with-function-overloading-in-rust-why-it-matters/)
//! in ergonomic function overloading in Rust. Technical details for the current stage of the
//! experiment can be found
//! [on the Inside Rust blog](https://blog.rust-lang.org/inside-rust/2026/08/19/overloading-experiment/).
//!
//! The `overload!` macro improves the ergonomics of the
//! [`splat` Rust language experiment](https://github.com/rust-lang/rust/issues/153629), by
//! allowing functions to be declared as an overload set. This requires a recent nightly Rust
//! compiler.
//!
//! # Example
//!
//! The macro supports methods with return values:
//! ```rust
//! #![feature(splat, tuple_trait)]
//! #![allow(incomplete_features, unused_braces)]
//! # #[macro_use] extern crate splat_overload;
//! use splat_overload::overload;
//! struct Calculator;
//! overload! {
//!     impl Calculator {
//!         fn compute(&self, x: i32) -> i32 { x * 2 }
//!         fn compute(&self, x: i32, y: i32) -> i32 { x + y }
//!     }
//! }
//! # fn main() {
//! let calc = Calculator;
//! assert_eq!(calc.compute(21), 42);
//! assert_eq!(calc.compute(10, 32), 42);
//! # }
//! ```
//!
//! And free functions:
//! ```rust
//! #![feature(splat, tuple_trait)]
//! #![allow(incomplete_features, unused_braces)]
//! # #[macro_use] extern crate splat_overload;
//! use splat_overload::overload;
//! overload! {
//!     fn compute(x: i32) -> i32 { x * 2 }
//!     fn compute(x: i32, y: i32) -> i32 { x + y }
//! }
//! # fn main() {
//! assert_eq!(compute(21), 42);
//! assert_eq!(compute(10, 32), 42);
//! # }
//! ```
//!
//! FIXME: document the resulting template we're trying to generate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, ItemFn, Pat, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

enum OverloadInput {
    /// A list of free functions
    Functions(Vec<ItemFn>),
    /// A list of methods or associated functions, with the impl (Self) type.
    /// FIXME: this should be refactored into separate Method and AssociatedFn variants
    Methods {
        /// The impl (Self) type.
        self_ty: syn::Ident,
        functions: Vec<ItemFn>,
    },
}

impl Parse for OverloadInput {
    /// Parse the input list of functions into different function kind variants.
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

/// Returns a unique trait name for the supplied function name.
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

/// Collects the argument types, names, and indices from a function signature.
fn collect_args(
    func: &ItemFn,
) -> (
    // FIXME: turn this into a vector of custom structs.
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

/// Returns the output type for a function signature, replacing the default type with `()`.
fn output_ty_for(func: &ItemFn) -> proc_macro2::TokenStream {
    match &func.sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    }
}

/// Transforms a list of argument types into a single tuple type containing those types.
fn tuple_ty_for(arg_types: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    if arg_types.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#arg_types),*,) }
    }
}

/// Generates the overladed trait code for a list of free functions.
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
        #[diagnostic::on_unimplemented(
            message = "missing overload for arguments `{Self}`",
            label = "the argument types `{Self}` do not match any overload",
            note = "check for missing or extra arguments, and check argument types",
            note = "consider adding a new overload in the overload! {{ ... }} block",
        )]
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

/// The kind of receiver for a method or associated function.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ReceiverKind {
    /// self
    Owned,
    /// &self
    Ref,
    /// &mut self
    RefMut,
    /// An associated function with no receiver.
    NoReceiver,
}

impl ReceiverKind {
    /// Returns the receiver kind for a function argument.
    /// This should be the first argument of the function, which is the only argument that can be
    /// a receiver.
    /// FIXME: take an iterator here, so we can ensure it's the first argument.
    fn from_args(r: &Option<&FnArg>) -> Self {
        match r {
            Some(FnArg::Receiver(r)) => {
                if r.reference.is_some() {
                    if r.mutability.is_some() {
                        Self::RefMut
                    } else {
                        Self::Ref
                    }
                } else {
                    Self::Owned
                }
            }
            None | Some(FnArg::Typed(_)) => Self::NoReceiver,
        }
    }
}

/// Generates the overladed trait code for a list of methods or associated functions.
fn generate_methods(self_ty: syn::Ident, functions: Vec<ItemFn>) -> TokenStream {
    let fn_name = &functions[0].sig.ident;
    let trait_name = trait_name_for(fn_name);

    // Check all the receiver kinds match, if not, panic.
    let receiver_kind = {
        let first_receiver = ReceiverKind::from_args(&functions[0].sig.inputs.first());
        for func in &functions {
            let next_receiver = ReceiverKind::from_args(&func.sig.inputs.first());
            if next_receiver != first_receiver {
                panic!(
                    "all overloads must use the same receiver kind (&self, &mut self, self, or no receiver)"
                );
            }
        }
        first_receiver
    };

    // Generate fragments used to generate the trait impmentation.
    // The generic type used for the `this` argument pseudo-receiver.
    let this_generic_ty = match receiver_kind {
        ReceiverKind::RefMut => quote! { , this: &mut R },
        ReceiverKind::Ref => quote! { , this: &R },
        ReceiverKind::Owned => quote! { , this: R },
        ReceiverKind::NoReceiver => quote! {},
    };

    // The concrete type used for the `this` argument pseudo-receiver.
    let this_concrete_ty = match receiver_kind {
        ReceiverKind::RefMut => quote! { , this: &mut #self_ty },
        ReceiverKind::Ref => quote! { , this: &#self_ty },
        ReceiverKind::Owned => quote! { , this: #self_ty },
        ReceiverKind::NoReceiver => quote! {},
    };

    let mut impls = Vec::new();
    let mut hidden_methods = Vec::new();

    for (i, func) in functions.iter().enumerate() {
        let (arg_types, arg_names, arg_indices) = collect_args(func);
        let output_ty = output_ty_for(func);
        let tuple_ty = tuple_ty_for(&arg_types);
        let block = &func.block;

        // The declaration receiver for the current function, already comes with &/&mut.
        let func_receiver = match func.sig.inputs.first() {
            Some(FnArg::Receiver(r)) => quote! { #r, },
            _ => quote! {},
        };
        // The receiver variable/type in the caller, either `this.` or the `Self::` type for
        // no receiver.
        let caller_receiver = match receiver_kind {
            ReceiverKind::RefMut | ReceiverKind::Ref | ReceiverKind::Owned => quote! { this. },
            ReceiverKind::NoReceiver => quote! { #self_ty:: },
        };

        // The hidden impl method name for the current function.
        let hidden_name = quote::format_ident!("__{}_impl_{}", fn_name, i);
        hidden_methods.push(quote! {
            fn #hidden_name(#func_receiver #(#arg_names: #arg_types),*) -> #output_ty {
                #block
            }
        });

        // The full trait impl for this specific function overload.
        impls.push(quote! {
            impl #trait_name<#self_ty> for #tuple_ty {
                type Output = #output_ty;
                fn call(self #this_concrete_ty) -> Self::Output {
                    #(let #arg_names = #arg_indices;)*
                    #caller_receiver #hidden_name(#(#arg_names),*)
                }
            }
        });
    }

    // The receiver for the overload dispatch function, its comma, and the receiver variable passed
    // to it in the caller (either `self` or no argument for associated functions).
    let (receiver, receiver_comma, self_arg) = match functions[0].sig.inputs.first() {
        Some(FnArg::Receiver(r)) => (quote! { #r }, quote! { , }, quote! { self }),
        _ => (quote! {}, quote! {}, quote! {}),
    };

    let generated = quote! {
        #[diagnostic::on_unimplemented(
            message = "missing overload for arguments `{Self}`",
            label = "the argument types `{Self}` do not match any overload",
            note = "check for missing or extra arguments, and check argument types",
            note = "consider adding a new overload in the overload! {{ ... }} block",
        )]
        trait #trait_name<R>: std::marker::Tuple {
            type Output;
            fn call(self #this_generic_ty) -> Self::Output;
        }

        #(#impls)*

        impl #self_ty {
            #(#hidden_methods)*

            fn #fn_name<T: #trait_name<Self>>(#receiver #receiver_comma #[rustc_splat] args: T) -> T::Output {
                args.call(#self_arg)
            }
        }
    };

    generated.into()
}

/// The main proc macro entry point.
#[proc_macro]
pub fn overload(input: TokenStream) -> TokenStream {
    // Parse the input list of functions into different function kind variants, then generate
    // the overloads for them.
    match parse_macro_input!(input as OverloadInput) {
        OverloadInput::Functions(functions) => generate_free_functions(functions),
        OverloadInput::Methods { self_ty, functions } => generate_methods(self_ty, functions),
    }
}
