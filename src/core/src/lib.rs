use axum::{body::Body, extract::Extension, http::StatusCode, routing::get};
use http::{request::Request, response::Response};
use std::any::Any;

use macros::{define_transformers, test_macro, transformer, use_transformer};

trait Transformer {
    type ClientType;
    fn describe(&self) -> &str;
}

#[derive(Debug, Clone)]
#[transformer]
struct TestType;

#[derive(Debug, Clone)]
#[transformer]
struct TestType2;

pub type TransformerGetter = Box<dyn FnOnce(i32) -> Box<dyn Any>>;

// trait Client {}
struct Client1;
struct Client2;
// impl Client for Client1 {}
// impl Client for Client2 {}
// pub enum Router {
//     TestType(TransformerGetter),
//     TestType2(TransformerGetter),
// }
pub struct Router(TransformerGetter);

impl Transformer for TestType {
    type ClientType = Client1;
    fn describe(&self) -> &str {
        "type1"
    }
}
impl Transformer for TestType2 {
    type ClientType = Client2;
    fn describe(&self) -> &str {
        "type2"
    }
}

// pub fn get_transformer(v: Router) -> impl Transformer {
//     // match v {
//     //     Router::TestType(v) => {
//     //         return *(*v)(32).downcast::<TestType>().unwrap();
//     //     }
//     //     Router::TestType2(v) => {
//     //         return *(*v)(64).downcast::<TestType2>().unwrap();
//     //     }
//     // }
// }

// pub fn get_TestType(v: Router) -> impl Transformer {
//     let Router(v) = v;
//     *(*v)(32).downcast::<TestType>().unwrap();
// }

// macro_rules! get_transformer {
//     ($k:expr, $v:expr, $input:expr) => {
//         *(*$v)($input).downcast::<$k>().unwrap()
//     };
// }

async fn handle_request<T>(
    Extension(handler): Extension<T>,
    request: Request<Body>,
) -> Result<Response<String>, StatusCode>
where
    T: Transformer + Send + Sync,
{
    Err(StatusCode::ACCEPTED)
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(declare_endpoints().await?.into_make_service())
        .await?;

    Ok(())

    // println!(
    //     "{:?}",
    //     TRANSFORMERS.iter().map(|(k, _v)| k).collect::<Vec<&&str>>()
    // );

    // for (k, Router(v)) in TRANSFORMERS.into_iter() {
    //     #[transformer_getter(k, v)]
    //     fn get
    // }
}

pub async fn declare_endpoints<T: Clone + Send + Sync + 'static>(
) -> Result<axum::Router<T>, Box<dyn std::error::Error>> {
    define_transformers!();

    let mut router = axum::Router::new();
    for (k, Router(v)) in TRANSFORMERS.into_iter() {
        // let v = get_transformer!(k, v, 32);
        // println!("{k}: {:?}", v.describe());
        router = router.route(format!("/{}", k).as_str(), use_transformer!(k, v));
    }
    Ok(router)
}
