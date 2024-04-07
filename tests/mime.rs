#[cfg(feature = "fs")]
mod tests {
    use async_std::fs;
    use async_std::io;
    use http_types_rs::{mime, Body, Response};

    #[async_std::test]
    async fn guess_plain_text_mime() -> io::Result<()> {
        let body = Body::from_path("tests/fixtures/index.html").await?;
        let mut res = Response::new(200);
        res.set_body(body);
        assert_eq!(res.content_type().map(|ct| ct.essence().to_ascii_lowercase()), Some("text/html".into()));
        Ok(())
    }

    #[async_std::test]
    async fn guess_binary_mime() -> http_types_rs::Result<()> {
        let body = Body::from_path("tests/fixtures/nori.png").await?;
        let mut res = Response::new(200);
        res.set_body(body);
        assert_eq!(res.content_type(), Some(mime::PNG));

        // Assert the file is correctly reset after we've peeked the bytes
        let left = fs::read("tests/fixtures/nori.png").await?;
        let right = res.body_bytes().await?;
        assert_eq!(left, right);
        Ok(())
    }

    #[async_std::test]
    async fn guess_mime_fallback() -> io::Result<()> {
        let body = Body::from_path("tests/fixtures/unknown.custom").await?;
        let mut res = Response::new(200);
        res.set_body(body);
        assert_eq!(res.content_type(), Some(mime::BYTE_STREAM));
        Ok(())
    }

    #[async_std::test]
    async fn parse_empty_files() -> http_types_rs::Result<()> {
        let body = Body::from_path("tests/fixtures/empty.custom").await?;
        let mut res = Response::new(200);
        res.set_body(body);
        assert_eq!(res.content_type(), Some(mime::BYTE_STREAM));
        Ok(())
    }

    // #[test]
    // fn match_mime_types() {
    //     let req = Request::get("https://example.com");
    //     match req.content_type() {
    //         Some(mime::JSON) => {}
    //         _ => {}
    //     }
    // }
}
