#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::*;

use std::result::Result as StdResult;
use std::sync::Mutex;

lazy_static! {
    static ref IMPLS: Mutex<Vec<String>> = Mutex::new(Default::default());
}

fn get_type_string_ts(s: &String) -> TokenStream {
    format!("string_{}", s).parse().unwrap()
}

#[proc_macro_attribute]
pub fn transformer(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    assert!(attr.is_empty());

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

    let v = IMPLS.lock().unwrap();
    let transformer_types = v.iter().map(|type_| {
        let s: &str = type_.as_str();
        let key: TokenStream = get_type_string_ts(type_);
        quote!(const #key: &str = #s;)
    });
    let transformer_kvs = v.iter().map(|type_| {
        let key = get_type_string_ts(type_);
        let type_: TokenStream = type_.parse().unwrap();
        quote!((#key, Router(::std::boxed::Box::new(|_i| { ::std::boxed::Box::new(#type_ {}) } ) )))
    });

    quote!(
        #(#transformer_types)*
        let TRANSFORMERS: ::std::collections::BTreeMap<&str, Router> = ::std::collections::BTreeMap::from([
            #(#transformer_kvs),*
        ]);
    )
    .into()
}

#[proc_macro]
pub fn test_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!(
        const I: &str = "abc";
    )
    .into()
}

#[proc_macro]
pub fn use_transformer(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let input = TokenStream::from(input);

    fn check_impl(
        impl_: Punctuated<Expr, Token![,]>,
    ) -> StdResult<(ExprPath, ExprPath), Box<dyn ToTokens>> {
        println!("{}", impl_.len());
        if impl_.len() != 2 {
            return Err(Box::new(impl_));
        }
        let k = match impl_.first().unwrap() {
            Expr::Path(p) => p,
            _ => return Err(Box::new(impl_.first().unwrap().clone())),
        };
        let v = match impl_.last().unwrap() {
            Expr::Path(p) => p,
            _ => return Err(Box::new(impl_.last().unwrap().clone())),
        };

        Ok((k.clone(), v.clone()))
    }

    println!("{}", input.to_string());

    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let impl_ = match parser.parse(input) {
        Ok(v) => v,
        Err(e) => {
            return Error::new_spanned(
                e.to_string().as_str(),
                "Unable to parse input (expected `key, value` pair)",
            )
            .to_compile_error()
            .into();
        }
    };
    // let impl_ = parse_macro_input!(input as Punctuated<Expr, Token![,]>);
    let (type_, callback_) = match check_impl(impl_) {
        Ok(t) => t,
        Err(e) => {
            return Error::new_spanned(e, "Unable to parse input (expected `key, value` pair)")
                .to_compile_error()
                .into()
        }
    };

    let v = IMPLS.lock().unwrap();
    let transformer_handlers = v.iter().map(|key| {
        let key_str = key.as_str(); //get_type_string_ts(key);
        let key_type: TokenStream = key.parse().unwrap();
        quote!(
            #key_str => {
                let transformer = *#callback_(32).downcast::<#key_type>().unwrap();
                get(handle_request::<#key_type>).layer(Extension(transformer))
            },
        )
    });

    println!("{}", transformer_handlers.clone().count());

    quote!(
        match #type_ {
            #(#transformer_handlers)*
            &_ => panic!()
        }
    )
    .into()
}
