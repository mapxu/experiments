use macros::{define_transformers, transformer};

trait Transformer {
    // type ClientType;
    fn describe(&self) -> &str;
}

#[derive(Debug)]
#[transformer]
struct TestType;

#[derive(Debug)]
#[transformer]
struct TestType2;

pub type TransformerGetter = Box<dyn FnOnce() -> Box<dyn Transformer>>;

trait Client {}
struct Client1;
struct Client2;
impl Client for Client1 {}
impl Client for Client2 {}
pub struct Router(TransformerGetter);

impl Transformer for TestType {
    // type ClientType = Client1;
    fn describe(&self) -> &str {
        "type1"
    }
}
impl Transformer for TestType2 {
    // type ClientType = Client2;
    fn describe(&self) -> &str {
        "type2"
    }
}

pub fn run() {
    define_transformers!();

    println!(
        "{:?}",
        TRANSFORMERS.iter().map(|(k, _v)| k).collect::<Vec<&&str>>()
    );

    for (k, Router(v)) in TRANSFORMERS.into_iter() {
        println!("{k}: {:?}", v().describe());
    }
}
