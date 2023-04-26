use macros::{define_transformers, transformer};

trait Transformer {
    type Sport: ?Sized;
    fn describe(&self) -> &str;
}

#[derive(Debug)]
#[transformer]
struct TestType;
// impl TestType {
//     pub fn new(int: i32) -> Self {
//         println!("{int}");
//         Self {}
//     }
// }

#[derive(Debug)]
#[transformer]
struct TestType2;
// impl TestType2 {
//     pub fn new(int: i32) -> Self {
//         println!("{int}");
//         Self {}
//     }
// }

pub type TransformerGetter<T> = Box<dyn FnOnce(i32) -> Box<dyn Transformer<Sport = T>>>;

trait Client {}
struct Client1;
struct Client2;
impl Client for Client1 {}
impl Client for Client2 {}
pub struct Router<S>(TransformerGetter<Box<S>>);

impl Transformer for TestType {
    type Sport = Box<Client1>;
    fn describe(&self) -> &str {
        "type1"
    }
}
impl Transformer for TestType2 {
    type Sport = Box<Client2>;
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
        println!("{k}: {:?}", v.describe());
    }

    /*
    fn getTestType(int: i32) -> impl Transformer {
        TestType {}
    }

    fn getTestType2(int: i32) -> impl Transformer {
        TestType2 {}
    }

    println!("{}", getTestType().describe());

    let table = ::std::collections::BTreeMap::from([
        ("TestType", getTestType),
        ("TestType2", getTestType2),
    ]);

    for (k, v) in table.into_iter() {
        println!("{k}: {:?}", v.describe());
    }
    */
}
