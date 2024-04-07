use http_types_rs::{url::Url, Method};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "serde")]
struct Params {
    msg: String,
}

#[derive(Deserialize)]
#[serde(crate = "serde")]
struct OptionalParams {
    _msg: Option<String>,
    _time: Option<u64>,
}

#[test]
fn successfully_deserialize_query() {
    let req = http_types_rs::Request::new(Method::Get, Url::parse("http://example.com/?msg=Hello").unwrap());

    let params = req.query::<Params>();
    assert!(params.is_ok());
    assert_eq!(params.unwrap().msg, "Hello");
}

#[test]
fn unsuccessfully_deserialize_query() {
    let req = http_types_rs::Request::new(Method::Get, Url::parse("http://example.com/").unwrap());

    let params = req.query::<Params>();
    assert!(params.is_err());
    assert_eq!(params.err().unwrap().to_string(), "missing field `msg`");
}

#[test]
fn malformatted_query() {
    let req = http_types_rs::Request::new(Method::Get, Url::parse("http://example.com/?error=should_fail").unwrap());

    let params = req.query::<Params>();
    assert!(params.is_err());
    assert_eq!(params.err().unwrap().to_string(), "missing field `msg`");
}

#[test]
fn empty_query_string_for_struct_with_no_required_fields() {
    let req = http_types_rs::Request::new(Method::Get, Url::parse("http://example.com").unwrap());

    let params = req.query::<OptionalParams>();
    assert!(params.is_ok());
}
