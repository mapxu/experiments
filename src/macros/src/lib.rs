#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
// use syn::parse::{ParseStream, Parser};
use syn::*;

// use std::collections::BTreeMap;
// use std::iter;
use std::sync::Mutex;

lazy_static! {
    static ref IMPLS: Mutex<Vec<String>> = Mutex::new(Default::default());
}

#[proc_macro_attribute]
pub fn transformer(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    assert!(attr.is_empty());

    // fn check_impl(impl_: ItemImpl) -> std::result::Result<Type, Box<dyn ToTokens>> {
    //     Ok(*impl_.self_ty)
    // }
    fn check_impl(impl_: ItemStruct) -> std::result::Result<Ident, Box<dyn ToTokens>> {
        Ok(impl_.ident)
    }

    let item2 = item.clone();
    let impl_ = parse_macro_input!(item2 as ItemStruct);

    let type_ = match check_impl(impl_) {
        Ok(v) => v,
        Err(e) => {
            return Error::new_spanned(e, "expected `impl Transformer for ...`")
                .to_compile_error()
                .into()
        }
    };

    println!("{}", type_.to_token_stream().to_string());

    let mut v = IMPLS.lock().unwrap();
    v.push(type_.to_token_stream().to_string());

    item
}

#[proc_macro]
pub fn define_transformers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assert!(input.is_empty());

    // let typeid = &quote!(::std::any::TypeId);

    let v = IMPLS.lock().unwrap();
    let elems = v.iter().map(|type_| {
        let key = type_.as_str();
        let type_: TokenStream = type_.parse().unwrap();
        quote!((#key, ::std::boxed::Box::new(|_i| Router ( ::std::boxed::Box::new(#type_ {}) ) ) ))
    });

    quote!(
        let TRANSFORMERS: ::std::collections::BTreeMap<&str, Router<Box<dyn Client>>> = ::std::collections::BTreeMap::from([
            #(#elems),*
        ]);
    )
    .into()
}
