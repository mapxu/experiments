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
pub struct Router(TransformerGetter);

struct Client1;
struct Client2;

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
}

pub async fn declare_endpoints<T: Clone + Send + Sync + 'static>(
) -> Result<axum::Router<T>, Box<dyn std::error::Error>> {
    define_transformers!();

    let mut router = axum::Router::new();
    for (k, Router(v)) in TRANSFORMERS.into_iter() {
        router = router.route(format!("/{}", k).as_str(), use_transformer!(k, v));
    }
    Ok(router)
}
