use futures::{executor::block_on, AsyncReadExt};
use http_types_rs::{Body, Method, Request, Response, StatusCode, Url};

#[test]
fn test_req_res_set_body() {
    let mut req = Request::new(Method::Get, Url::parse("http://example.com/").unwrap());
    req.set_body(Body::empty());
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(req);
    let body = block_on(async move {
        let mut body = Vec::new();
        res.read_to_end(&mut body).await.unwrap();
        body
    });
    assert!(body.is_empty());
}

#[test]
fn test_req_res_take_replace_body() {
    let mut req = Request::new(Method::Get, Url::parse("http://example.com/").unwrap());
    req.take_body();
    let mut res = Response::new(StatusCode::Ok);
    res.replace_body(req);
    let body = block_on(async move {
        let mut body = Vec::new();
        res.read_to_end(&mut body).await.unwrap();
        body
    });
    assert!(body.is_empty());
}
